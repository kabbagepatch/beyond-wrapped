use std::{
  fs::File, io::Read, sync::Mutex, time::{SystemTime, UNIX_EPOCH}, vec,
};
use rusqlite::Connection;
use sha2::{Digest, Sha256};
use tauri::{AppHandle, Manager};
use tauri::async_runtime::spawn_blocking;
use tauri_plugin_store::StoreExt;
use zip::{result::ZipError, ZipArchive};

use crate::{
  AppError::MyError, db::{connection::init_db, queries::{get_top_items_range, get_track_plays, get_track_stats, insert_extended_history}}, file::{get_raw_history_files, remove_incoming_dir, rename_incoming_dir, rename_raw_dir, save_raw_track_data}, models::{Play, PlayCount, Stats}, processing::process_raw_history_file,
};

mod file;
mod models;
mod processing;
mod db;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
  #[error("{0}")]
  MyError(String),
  #[error("Tauri error: {0}")]
  Tauri(#[from] tauri::Error),
  #[error("IO error: {0}")]
  Io(#[from] std::io::Error),
  #[error("ZIP error: {0}")]
  Zip(#[from] ZipError),
  #[error("JSON error: {0}")]
  Json(#[from] serde_json::Error),
  #[error("Task join error: {0}")]
  Join(String),
  #[error("Tauri Store error: {0}")]
  Store(#[from] tauri_plugin_store::Error),
  #[error("Chrono Parse error: {0}")]
  Chrono(#[from] chrono::ParseError),
  #[error("Database Error: {0}")]
  Resqlite(#[from] rusqlite::Error),
  #[error("Database error")]
  SqlError(),
}

impl serde::Serialize for AppError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn process_zip_file(app: AppHandle, file_path: String) -> Result<String, AppError> {
  println!("Received file path: {}", file_path);
  let store = app.store("store.json")?;
  store.set("full-history-processed", false);
  store.save()?;
  
  let result = spawn_blocking(move || -> Result<String, AppError> {
    remove_incoming_dir(&app)?;

    let conn = app.state::<Mutex<Connection>>();
    let mut conn = conn.lock().unwrap_or_else(|e| e.into_inner());
    let transaction = conn.transaction()?;
    let file = File::open(&file_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut saved = 0;
    for i in 0..archive.len() {
      let mut file = archive.by_index(i)?;
      if file.is_file() {
        let file_name = file.name().to_string().rsplit('/').next().unwrap().to_string();

        if file_name.ends_with(".json") {
          println!("Parsing {}", file_name);
          let mut buf = Vec::new();
          file.read_to_end(&mut buf)?;
          let content_hash = hex::encode(Sha256::digest(&buf));
          let data: Vec<models::RawTrackData> = serde_json::from_slice(&buf)?;

          if let Err(e) = save_raw_track_data(&app, &content_hash, &data) {
            eprintln!("Error saving {} to raw history: {}", file_name, e);
            return Err(MyError("There was an error processing the zip file".to_string()));
          }
          insert_extended_history(&transaction, &content_hash, Some(&file_name))?;
          saved += 1;
        }
      }
    }
    if saved == 0 {
      return Err(MyError("No listening history was found in that archive".to_string()));
    }

    rename_raw_dir(&app)?;
    rename_incoming_dir(&app)?;
    transaction.commit()?;

    let store = app.store("store.json")?;
    store.set("last-upload-history", SystemTime::now().duration_since(UNIX_EPOCH).expect("bad").as_millis() as u64);
    store.save()?;

    Ok::<_, AppError>(format!(
      "Successfully read {} files from archive",
      saved
    ))
  })
  .await
  .map_err(|e| AppError::Join(e.to_string()))??;

  Ok(result)
}

#[tauri::command]
async fn process_raw_history(app: AppHandle) -> Result<String, AppError> {
  println!("Received process request");
  let store = app.store("store.json")?;
  let already_processed = store.get("full-history-processed").and_then(|v| v.as_bool()).unwrap_or(false);
  if already_processed {
    println!("Extended History already processed");
    return Ok::<_, AppError>("Extended History already processed".to_string());
  }

  let result: String = spawn_blocking(move || -> Result<String, AppError> {
    let raw_json_files = get_raw_history_files(&app)?;
    for raw_entry in &raw_json_files {
      if let Err(e) = process_raw_history_file(&app, raw_entry) {
        return Err(MyError(format!("There was an issue processing your raw history file: {}", e)));
      };
    }

    store.set("full-history-processed", true);
    store.save()?;

    Ok::<_, AppError>(format!(
      "Successfully processed {} files from raw history",
      &raw_json_files.len()
    ))
  })
  .await
  .map_err(|e| AppError::Join(e.to_string()))??;

  Ok(result)
}

#[tauri::command]
async fn get_top_items(app: AppHandle, item: &str, year: u32, month: Option<u32>) -> Result<Vec<PlayCount>, AppError> {
  let conn = app.state::<Mutex<Connection>>();
  let conn = conn.lock().unwrap_or_else(|e| e.into_inner());

  let from = if month.is_some() { let month = month.unwrap(); format!("{year}-{month:02}-01T00:00:00Z") } else { format!("{year}-01-01T00:00:00Z") };
  let to = if month.is_some() { let month = month.unwrap(); format!("{year}-{month:02}-31T23:59:59Z") } else { format!("{year}-12-31T23:59:59Z") };

  get_top_items_range(&conn, item, &from, &to)
}

#[tauri::command]
async fn get_top_items_custom(app: AppHandle, item: &str, from_year: u32, from_month: u32, to_year: u32, to_month: u32) -> Result<Vec<PlayCount>, AppError> {
  let conn = app.state::<Mutex<Connection>>();
  let conn = conn.lock().unwrap_or_else(|e| e.into_inner());

  let from = format!("{from_year}-{from_month:02}-01T00:00:00Z");
  let to = format!("{to_year}-{to_month:02}-31T23:59:59Z");

  get_top_items_range(&conn, item, &from, &to)
}

#[tauri::command]
async fn get_track_plays_track(app: AppHandle, track: &str, artist: &str) -> Result<Vec<Play>, AppError> {
  let conn = app.state::<Mutex<Connection>>();
  let conn = conn.lock().unwrap_or_else(|e| e.into_inner());

  get_track_plays(&conn, Some(track), artist, None)
}

#[tauri::command]
async fn get_track_plays_artist(app: AppHandle, artist: &str) -> Result<Vec<Play>, AppError> {
  let conn = app.state::<Mutex<Connection>>();
  let conn = conn.lock().unwrap_or_else(|e| e.into_inner());

  get_track_plays(&conn, None, artist, None)
}

#[tauri::command]
async fn get_track_plays_album(app: AppHandle, album: &str, artist: &str) -> Result<Vec<Play>, AppError> {
  let conn = app.state::<Mutex<Connection>>();
  let conn = conn.lock().unwrap_or_else(|e| e.into_inner());

  get_track_plays(&conn, None, artist, Some(album))
}

#[tauri::command]
async fn get_track_stats_artist(app: AppHandle, artist: &str) -> Result<Vec<Stats>, AppError> {
  let conn = app.state::<Mutex<Connection>>();
  let conn = conn.lock().unwrap_or_else(|e| e.into_inner());

  get_track_stats(&conn, artist, None)
}

#[tauri::command]
async fn get_track_stats_album(app: AppHandle, album: &str, artist: &str) -> Result<Vec<Stats>, AppError> {
  let conn = app.state::<Mutex<Connection>>();
  let conn = conn.lock().unwrap_or_else(|e| e.into_inner());

  get_track_stats(&conn, artist, Some(album))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let conn = init_db(app);
      app.manage(Mutex::new(conn));
      Ok(())
    })
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_store::Builder::new().build())
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      greet,
      process_zip_file,
      process_raw_history,
      get_top_items,
      get_top_items_custom,
      get_track_plays_track,
      get_track_plays_artist,
      get_track_plays_album,
      get_track_stats_artist,
      get_track_stats_album
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
