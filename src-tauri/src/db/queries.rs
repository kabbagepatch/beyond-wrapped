use rusqlite::{Connection, params};

use crate::{AppError::{self}, db::connection::{execute, query_map, query_row}, models::{RawFile, TrackCount, TrackEntryData}};

pub fn get_extended_history_file_info(conn: &Connection, content_hash: &str) -> Result<Option<RawFile>, AppError> {
  let sql = "SELECT content_hash, filename, processed_at FROM extended_history_files where content_hash = ?1";
  query_row(&conn, sql, [content_hash], |row| {
      Ok(RawFile { content_hash: row.get(0)?, filename: row.get(1)?, processed_at: row.get(2)? })
  })
}

pub fn insert_extended_history(conn: &Connection, content_hash: &str, file_name: Option<&str>) -> Result<usize, AppError> {
  if file_name.is_some() {
    let sql = "
      INSERT INTO extended_history_files (content_hash, filename)
      VALUES (?1, ?2)
      ON CONFLICT(content_hash) DO NOTHING;
    ";
    execute(&conn, sql, [content_hash, file_name.unwrap()])
  } else {
    let sql = "INSERT INTO extended_history_files (content_hash) VALUES (?1)";
    execute(&conn, sql, [content_hash])
  }
}

pub fn insert_artist(conn: &Connection, entry: &TrackEntryData) -> Result<usize, AppError> {
  execute(
    &conn,
    "INSERT INTO artists (name) VALUES (?1) ON CONFLICT(name) DO NOTHING;",
    [entry.track.artist_name.clone()]
  )
}

pub fn insert_album(conn: &Connection, entry: &TrackEntryData) -> Result<usize, AppError> {
  execute(
    &conn,
    "INSERT INTO albums (name, artist) VALUES (?1, ?2) ON CONFLICT(name, artist) DO NOTHING;",
    [entry.track.album_name.clone(), entry.track.artist_name.clone()]
  )
}

pub fn insert_track(conn: &Connection, entry: &TrackEntryData) -> Result<usize, AppError> {
  execute(
    &conn,
    "
      INSERT INTO tracks (spotify_id, name, artist, album) 
      VALUES (?1, ?2, ?3, ?4)
      ON CONFLICT(spotify_id) DO NOTHING;
    ",
    [
      entry.track.id.clone(),
      entry.track.track_name.clone(),
      entry.track.artist_name.clone(),
      entry.track.album_name.clone()
    ]
  )
}

pub fn insert_play(conn: &Connection, entry: &TrackEntryData) -> Result<usize, AppError> {
  execute(
    &conn,
    "
        INSERT INTO plays (track_id, time_stamp, ms_played)
        VALUES (?1, ?2, ?3)
        ON CONFLICT(track_id, time_stamp) DO NOTHING;
    ",
    params![
      entry.track.id,
      entry.time_stamp,
      entry.ms_played
    ]
  )
}

pub fn get_top_items_range(conn: &Connection, item: &str, from: &str, to: &str) -> Result<Vec<TrackCount>, AppError> {
  let (select, group_by) = match item {
    "tracks"  => (
      "t.name, t.artist",
      "GROUP BY t.name, t.artist"
    ),
    "artists" => (
      "t.artist, t.name",
      "GROUP BY t.artist"
    ),
    "albums"  => (
      "t.album, t.artist",
      "GROUP BY t.album, t.artist"
    ),
    _ => return Err(AppError::MyError("Invalid resource type".to_string())),
  };
  let sql = format!("
    SELECT {select}, COUNT(*) AS play_count, SUM(p.ms_played) AS ms_played
    FROM plays p JOIN tracks t ON p.track_id = t.spotify_id
    WHERE p.time_stamp BETWEEN ?1 AND ?2
    {group_by}
    ORDER BY play_count DESC
  ");

  query_map(conn, &sql, [from, to], |row| Ok(
    TrackCount {
      primary: row.get(0)?,
      secondary: if item != "artists" { row.get(1)? } else { None },
      play_count: row.get(2)?,
      ms_played: row.get(3)?,
    }
  ))
}
