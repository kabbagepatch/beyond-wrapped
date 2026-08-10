use rusqlite::{Connection, params};

use crate::{AppError::{self}, db::connection::{execute, query_map, query_row}, models::{Album, Artist, Bounds, ItemPlayData, PlayEntry, RawFile, RawTrackEntryData, Track, TrackStats}};

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

pub fn insert_artist(conn: &Connection, entry: &RawTrackEntryData) -> Result<usize, AppError> {
  execute(
    &conn,
    "INSERT INTO artists (name) VALUES (?1) ON CONFLICT(name) DO NOTHING;",
    [entry.track.artist_name.clone()]
  )
}

pub fn insert_album(conn: &Connection, entry: &RawTrackEntryData) -> Result<usize, AppError> {
  execute(
    &conn,
    "INSERT INTO albums (name, artist) VALUES (?1, ?2) ON CONFLICT(name, artist) DO NOTHING;",
    [entry.track.album_name.clone(), entry.track.artist_name.clone()]
  )
}

pub fn insert_track(conn: &Connection, entry: &RawTrackEntryData) -> Result<usize, AppError> {
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

pub fn insert_play(conn: &Connection, entry: &RawTrackEntryData) -> Result<usize, AppError> {
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

pub fn get_top_items_range(conn: &Connection, item: &str, from: &str, to: &str) -> Result<Vec<ItemPlayData>, AppError> {
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
    ItemPlayData {
      primary: row.get(0)?,
      secondary: if item != "artists" { row.get(1)? } else { None },
      play_count: row.get(2)?,
      ms_played: row.get(3)?,
    }
  ))
}

pub fn get_track_plays(conn: &Connection, track: Option<&str>, artist: &str, album: Option<&str>) -> Result<Vec<PlayEntry>, AppError> {
  let (where_sql, params) = if track.is_none() && album.is_none() {
    ("t.artist = ?1", params![artist])
  } else if track.is_some() {
    ("t.name = ?1 AND t.artist = ?2", params![track.unwrap(), artist])
  } else {
    ("t.album = ?1 AND t.artist = ?2", params![album.unwrap(), artist])
  };

  let sql = format!("
    SELECT t.name, t.artist, t.album, p.time_stamp, p.ms_played
    FROM plays p JOIN tracks t ON p.track_id = t.spotify_id
    WHERE {where_sql}
    ORDER BY p.time_stamp ASC
  ");

  query_map(conn, &sql, params, |row| Ok(
    PlayEntry { track: row.get(0)?, artist: row.get(1)?, album: row.get(2)?, time_stamp: row.get(3)?, ms_played: row.get(4)? }
  ))
}

pub fn get_track_stats(conn: &Connection, artist: &str, album: Option<&str>) -> Result<Vec<TrackStats>, AppError> {
  let (where_sql, params) = if album.is_none() {
    ("t.artist = ?1", params![artist])
  } else {
    ("t.album = ?1 AND t.artist = ?2", params![album.unwrap(), artist])
  };

  let sql = format!("
    SELECT t.name, t.artist, t.album, COUNT(*) AS play_count, SUM(p.ms_played) AS ms_played, MIN(p.time_stamp) AS first_play
    FROM plays p JOIN tracks t ON p.track_id = t.spotify_id
    WHERE {where_sql}
    GROUP BY t.name
    ORDER BY play_count DESC
  ");

  query_map(conn, &sql, params, |row| Ok(
    TrackStats {
      track: row.get(0)?,
      artist: row.get(1)?,
      album: row.get(2)?,
      play_count: row.get(3)?,
      ms_played: row.get(4)?,
      first_play: row.get(5)?
    }
  ))
}

pub fn search_tracks(conn: &Connection, search_string: &str) -> Result<Vec<Track>, AppError> {
  let sql = "
    SELECT t.name, t.artist, t.album, COUNT(*) AS play_count
    FROM plays p JOIN tracks t ON p.track_id = t.spotify_id
    WHERE t.name LIKE ?1
    GROUP BY t.name, t.artist
    ORDER BY play_count DESC
    LIMIT 50
  ";

  query_map(conn, sql, params![format!("%{search_string}%")],|row| Ok(
    Track {
      name: row.get(0)?,
      artist: row.get(1)?,
      album: row.get(2)?,
      play_count: row.get(3)?,
    }
  ))
}

pub fn search_artists(conn: &Connection, search_string: &str) -> Result<Vec<Artist>, AppError> {
  let sql = "
    SELECT t.artist, COUNT(*) AS play_count
    FROM plays p JOIN tracks t ON p.track_id = t.spotify_id
    WHERE t.artist LIKE ?1
    GROUP BY t.artist
    ORDER BY play_count DESC
    LIMIT 50
  ";

  query_map(conn, sql, params![format!("%{search_string}%")],|row| Ok(
    Artist { name: row.get(0)?, play_count: row.get(1)? }
  ))
}

pub fn search_albums(conn: &Connection, search_string: &str) -> Result<Vec<Album>, AppError> {
  let sql = "
    SELECT t.album, t.artist, COUNT(*) AS play_count
    FROM plays p JOIN tracks t ON p.track_id = t.spotify_id
    WHERE t.album LIKE ?1
    GROUP BY t.album, t.artist
    ORDER BY play_count DESC
    LIMIT 50
  ";

  query_map(conn, sql, params![format!("%{search_string}%")],|row| Ok(
    Album { name: row.get(0)?, artist: row.get(1)?, play_count: row.get(2)? }
  ))
}

pub fn get_play_bounds(conn: &Connection) -> Result<Option<Bounds>, AppError> {
  let sql = "
    SELECT MIN(p.time_stamp) as min_timestamp, MAX(p.time_stamp) as max_timestamp
    FROM plays p
  ";

  query_row(conn, sql, [], |row| Ok(
    Bounds { min_timestamp: row.get(0)?, max_timestamp: row.get(1)? }
  ))
}
