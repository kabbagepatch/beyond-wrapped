use std::{path::PathBuf, sync::Mutex};

use rusqlite::{Connection};
use tauri::{AppHandle, Manager};

use crate::{
  AppError::{self}, db::queries::{get_extended_history_file_info, insert_album, insert_artist, insert_extended_history, insert_play, insert_track}, file::get_raw_track_data,
};

pub fn process_raw_history_file(app: &AppHandle, filepath: &PathBuf) -> Result<(), AppError> {
  let content_hash = filepath.file_name().unwrap().to_string_lossy();

  let conn = app.state::<Mutex<Connection>>();
  let mut conn = conn.lock().unwrap_or_else(|e| e.into_inner());

  let file_info = get_extended_history_file_info(&conn, &content_hash)?;
  let file_name = file_info
    .as_ref()
    .and_then(|info| info.filename.clone())
    .unwrap_or_else(|| content_hash.to_string());
  match &file_info {
    Some(info) => {
      if info.processed_at.is_some() {
        println!("{} already processed", file_name);
        return Ok(());
      }
    }
    None => {
      insert_extended_history(&conn, &content_hash, None)?;
    }
  }

  println!("Processing {}", file_name);
  let raw_data = match get_raw_track_data(filepath.as_path()) {
    Ok(c) => c,
    Err(err) => {
      eprintln!("Failed to read {}: {}", file_name, err);
      return Ok(());
    }
  };

  let transaction = conn.transaction()?;
  for entry in raw_data {
    if entry.ms_played < 30000 {
      continue;
    }

    insert_artist(&transaction, &entry)?;
    insert_album(&transaction, &entry)?;
    insert_track(&transaction, &entry)?;
    insert_play(&transaction, &entry)?;
  }

  let sql = "UPDATE extended_history_files SET processed_at = CURRENT_TIMESTAMP WHERE content_hash = ?1";
  transaction.execute(sql, (content_hash,))?;
  transaction.commit()?;

  Ok(())
}
