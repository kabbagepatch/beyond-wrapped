use std::{
  fs::{self},
  path::{Path, PathBuf},
};

use tauri::{Error, Manager};

use crate::models::{RawTrackDataSpotify, RawTrackData, RawTrackEntryData};

const RAW_HISTORY: &str = "raw_history";
const RAW_HISTORY_INCOMING: &str = "raw_history.incoming";
const RAW_HISTORY_BACKUP: &str = "raw_history.back";

pub fn get_raw_history_files(app: &tauri::AppHandle) -> Result<Vec<PathBuf>, Error> {
  let raw_history_dir = app.path().app_data_dir()?.join(RAW_HISTORY);
  let raw_json_files = fs::read_dir(raw_history_dir)?
    .filter_map(|entry| entry.ok())
    .map(|entry| entry.path())
    .collect();

  Ok(raw_json_files)
}

pub fn get_raw_track_data(file_path: &Path) -> Result<Vec<RawTrackEntryData>, Error> {
  let contents = fs::read(file_path)?;
  let raw_track_data: Vec<RawTrackDataSpotify> = serde_json::from_slice(&contents)?;
  let track_data: Vec<RawTrackEntryData> = raw_track_data
    .into_iter()
    .filter_map(|e: RawTrackDataSpotify| {
      return Some(RawTrackEntryData {
        track: RawTrackData {
          id: e.spotify_track_uri?,
          track_name: e.master_metadata_track_name?,
          artist_name: e.master_metadata_album_artist_name?,
          album_name: e.master_metadata_album_album_name?,
        },
        time_stamp: e.ts?,
        ms_played: e.ms_played?,
      });
    })
    .collect();

  Ok(track_data)
}

pub fn save_raw_track_data(
  app: &tauri::AppHandle,
  file_name: &str,
  data: &Vec<RawTrackDataSpotify>,
) -> Result<(), Error> {
  let dir = app.path().app_data_dir()?.join(RAW_HISTORY_INCOMING);
  fs::create_dir_all(&dir)?;
  let json = serde_json::to_string_pretty(&data)?;
  let file_path = dir.join(&file_name);
  fs::write(file_path, json)?;

  Ok(())
}

pub fn rename_dir(app: &tauri::AppHandle, from: &str, to: &str) -> Result<(), Error> {
  let base = app.path().app_data_dir().unwrap();
  let temp_dir = base.join(to);
  let dir = base.join(from);
  if fs::exists(&temp_dir)? {
    fs::remove_dir_all(&temp_dir)?;
  }
  if fs::exists(&dir)? {
    fs::rename(&dir, &temp_dir)?;
  }

  Ok(())
}

pub fn remove_dir(app: &tauri::AppHandle, dir_name: &str) -> Result<(), Error> {
  let base = app.path().app_data_dir().unwrap();
  let dir = base.join(dir_name);
  if fs::exists(&dir)? {
    fs::remove_dir_all(&dir)?;
  }

  Ok(())
}

pub fn remove_incoming_dir(app: &tauri::AppHandle) -> Result<(), Error> {
  remove_dir(app, RAW_HISTORY_INCOMING)
}

pub fn rename_incoming_dir(app: &tauri::AppHandle) -> Result<(), Error> {
  rename_dir(app, RAW_HISTORY_INCOMING, RAW_HISTORY)
}

pub fn rename_raw_dir(app: &tauri::AppHandle) -> Result<(), Error> {
  rename_dir(app, RAW_HISTORY, RAW_HISTORY_BACKUP)
}
