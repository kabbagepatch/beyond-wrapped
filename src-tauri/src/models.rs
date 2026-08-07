use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawTrackDataSpotify {
  pub spotify_track_uri: Option<String>,
  pub master_metadata_track_name: Option<String>,
  pub master_metadata_album_artist_name: Option<String>,
  pub master_metadata_album_album_name: Option<String>,
  pub ts: Option<String>,
  pub ms_played: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RawTrackData {
  pub id: String,
  pub track_name: String,
  pub artist_name: String,
  pub album_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawTrackEntryData {
  pub track: RawTrackData,
  pub time_stamp: String,
  pub ms_played: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RawFile {
  pub content_hash: String,
  pub filename: Option<String>,
  pub processed_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Track {
  pub name: String,
  pub artist: String,
  pub album: String,
  pub play_count: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Artist {
  pub name: String,
  pub play_count: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Album {
  pub name: String,
  pub artist: String,
  pub play_count: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchResults {
  pub tracks: Vec<Track>,
  pub artists: Vec<Artist>,
  pub albums: Vec<Album>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemPlayData {
  pub primary: String,
  pub secondary: Option<String>,
  pub play_count: i32,
  pub ms_played: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayEntry {
  pub track: String,
  pub artist: String,
  pub album: String,
  pub time_stamp: String,
  pub ms_played: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrackStats {
  pub track: String,
  pub artist: String,
  pub album: String,
  pub play_count: i32,
  pub ms_played:  i64,
  pub first_play: String,
}
