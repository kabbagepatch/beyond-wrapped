pub const SCHEMA: &str = r#"
BEGIN;
CREATE TABLE IF NOT EXISTS artists (
  name  TEXT PRIMARY KEY
);
CREATE TABLE IF NOT EXISTS albums (
  name    TEXT NOT NULL,
  artist  TEXT NOT NULL REFERENCES artists(name),
  PRIMARY KEY (name, artist)
);
CREATE TABLE IF NOT EXISTS tracks (
  spotify_id  TEXT PRIMARY KEY,
  name        TEXT NOT NULL,
  artist      TEXT NOT NULL,
  album       TEXT NOT NULL,
  play_count  INTEGER NOT NULL DEFAULT 0,
  FOREIGN KEY (album, artist) REFERENCES albums(name, artist)
);
CREATE TABLE IF NOT EXISTS plays (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  track_id    TEXT REFERENCES tracks(spotify_id),
  time_stamp  TEXT NOT NULL,
  ms_played   INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS extended_history_files (
  filename     TEXT PRIMARY KEY,
  processed_at TEXT
);
COMMIT;
"#;
