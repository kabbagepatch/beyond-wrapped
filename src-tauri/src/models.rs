use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawTrackData {
  pub spotify_track_uri: Option<String>,
  pub master_metadata_track_name: Option<String>,
  pub master_metadata_album_artist_name: Option<String>,
  pub master_metadata_album_album_name: Option<String>,
  pub ts: Option<String>,
  pub ms_played: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub ms_played: i32,
    pub play_count: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrackData {
    pub id: String,
    pub track_name: String,
    pub artist_name: String,
    pub album_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackEntryData {
    pub track: TrackData,
    pub time_stamp: String,
    pub ms_played: i32,
}

pub type EntryStats = HashMap<String, Entry>;
pub type MonthlyStats = HashMap<u32, EntryStats>;
pub type YearlyStats = HashMap<i32, MonthlyStats>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackPlay {
    pub time_stamp: String,
    pub ms_played: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrackStats {
    pub info: TrackData,
    pub plays: Vec<TrackPlay>,
}
pub type FullTrackStats = HashMap<String, TrackStats>;
pub type FullArtistStats = HashMap<String, FullTrackStats>;
pub type FullAlbumStats = HashMap<String, FullTrackStats>;
