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
  FOREIGN KEY (album, artist) REFERENCES albums(name, artist)
);
CREATE TABLE IF NOT EXISTS plays (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  track_id    TEXT NOT NULL REFERENCES tracks(spotify_id),
  time_stamp  TEXT NOT NULL,
  ms_played   INTEGER NOT NULL,
  UNIQUE(track_id, time_stamp)
);
CREATE TABLE IF NOT EXISTS extended_history_files (
  content_hash TEXT PRIMARY KEY,
  filename     TEXT,
  processed_at TEXT
);
CREATE INDEX IF NOT EXISTS idx_plays_track ON plays(track_id);
CREATE INDEX IF NOT EXISTS idx_plays_timestamp ON plays(time_stamp);
CREATE INDEX IF NOT EXISTS idx_tracks_name_artist ON tracks(name, artist);
CREATE INDEX IF NOT EXISTS idx_tracks_artist ON tracks(artist);
CREATE INDEX IF NOT EXISTS idx_tracks_album ON tracks(album, artist);
COMMIT;
"#;
