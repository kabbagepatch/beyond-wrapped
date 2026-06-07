use std::{fs::{self}, path::{Path, PathBuf}, str::FromStr};

use serde::{Serialize, de::DeserializeOwned};
use tauri::{Error, Manager};

use crate::models::{EntryStats, RawTrackData, TrackData, TrackEntryData};

const RAW_HISTORY: &str = "raw_history";
const PROCESSED_HISTORY: &str = "processed_history";

pub fn get_raw_history_files(app: &tauri::AppHandle) -> Result<Vec<PathBuf>, Error> {
    let raw_history_dir = app.path().app_data_dir()?.join(RAW_HISTORY);
    let raw_json_files = fs::read_dir(raw_history_dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension().and_then(|ext| ext.to_str()) == Some("json")
        })
        .collect();

    Ok(raw_json_files)
}

pub fn get_raw_track_data(file_path: &Path) -> Result<Vec<TrackEntryData>, Error> {
    let contents = fs::read(file_path)?;
    let raw_track_data: Vec<RawTrackData> = serde_json::from_slice(&contents)?;
    let track_data: Vec<TrackEntryData> = raw_track_data
        .into_iter()
        .filter_map(|e: RawTrackData| {
            return Some(TrackEntryData {
                track: TrackData {
                    id: e.spotify_track_uri?,
                    track_name: e.master_metadata_track_name?,
                    artist_name: e.master_metadata_album_artist_name?,
                    album_name: e.master_metadata_album_album_name?,
                },
                time_stamp: e.ts?,
                ms_played: e.ms_played?
            })
        })
        .collect();

    Ok(track_data)
}

pub fn save_raw_track_data(app: &tauri::AppHandle, file_name: &str, data: &Vec<RawTrackData>) -> Result<(), Error> {
    let dir = app.path().app_data_dir()?.join(RAW_HISTORY);
    fs::create_dir_all(&dir)?;
    let json = serde_json::to_string_pretty(&data)?;
    let file_path = dir.join(&file_name);
    fs::write(file_path, json)?;

    Ok(())
}

pub fn rename_dir(app: &tauri::AppHandle, dir_name: &str) -> Result<(), Error> {
    let base = app.path().app_data_dir().unwrap();
    let temp_dir = base.join(format!("{}.tmp", dir_name));
    let dir = base.join(dir_name);
    if fs::exists(&temp_dir)? {
        fs::remove_dir_all(&temp_dir)?;
    }
    if fs::exists(&dir)? {
        fs::rename(&dir, &temp_dir)?;
    }

    Ok(())
}

pub fn rename_raw_dir(app: &tauri::AppHandle) -> Result<(), Error> {
    rename_dir(app, RAW_HISTORY)
}

pub fn get_base_dir(app: &tauri::AppHandle, year: Option<i32>, month: Option<u32>) -> PathBuf {
    let base = app.path().app_data_dir().unwrap();
    let mut dir = base.join(PROCESSED_HISTORY);
    if let Some(year) = year {
        dir = dir.join(year.to_string());
    }
    if let Some(month) = month {
        dir = dir.join(month.to_string());
    }

    dir
}

pub fn rename_processed_dir(app: &tauri::AppHandle) -> Result<(), Error> {
    rename_dir(app, PROCESSED_HISTORY)
}

pub fn create_stats_dir(app: &tauri::AppHandle, year: i32, month: Option<u32>) -> Result<(), Error> {
    let dir = get_base_dir(&app, Some(year), month);
    fs::create_dir_all(&dir)?;

    Ok(())
}

pub fn get_dir_names<T>(app: &tauri::AppHandle, year: Option<i32>) -> Result<Vec<T>, Error> where T: FromStr {
    let dir = get_base_dir(app, year, None);
    let contents = fs::read_dir(dir)?;
    let dir_names: Vec<T> = contents
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| {
            entry.file_name()
                .to_str()?
                .parse::<T>()
                .ok()
        })
        .collect();

    Ok(dir_names)
}

pub fn get_years_dir_names(app: &tauri::AppHandle) -> Result<Vec<i32>, Error> {
    get_dir_names::<i32>(app, None)
}

pub fn get_months_dir_names(app: &tauri::AppHandle, year: i32) -> Result<Vec<u32>, Error> {
    get_dir_names::<u32>(app, Some(year))
}

pub fn rename_temp_file(app: &tauri::AppHandle, year: Option<i32>, month: Option<u32>, file_name: &str) {
    let result = (|| -> Result<(), Error> {
        let dir = get_base_dir(&app, year, month);
        let temp_file_path = dir.join(format!("{}.tmp", &file_name));
        let new_file_path = dir.join(&file_name);
        fs::rename(temp_file_path, new_file_path)?;

        Ok(())
    })();

    if let Err(e) = result {
        eprintln!("Unable to rename {}.tmp for {}-{}: {}", file_name, year.unwrap_or(0), month.unwrap_or(0), e);
    }
}

pub fn save_processed_data<T>(app: &tauri::AppHandle, year: i32, month: Option<u32>, file_name: &str, data: &T) where T: Serialize {
    let result = (|| -> Result<(), Error> {
        let dir = get_base_dir(&app, Some(year), month);
        fs::create_dir_all(&dir)?;
        let json = serde_json::to_string_pretty(&data)?;
        let file_path = dir.join(&file_name);
        fs::write(file_path, json)?;

        Ok(())
    })();

    if let Err(e) = result {
        eprintln!("Unable to save {} for {}-{}: {}", file_name, year, month.unwrap_or(0), e);
    }
}

pub fn get_saved_processed_data(app: &tauri::AppHandle, year: i32, month: Option<u32>, file_name: &str) -> Result<EntryStats, Error> {
    let dir = get_base_dir(&app, Some(year), month);
    let file_path = dir.join(&file_name);
    let contents = fs::read(file_path)?;
    let processed_data: EntryStats = serde_json::from_slice(&contents)?;

    Ok(processed_data)
}

pub fn get_full_stats<T>(app: &tauri::AppHandle, file_name: &str) -> Result<T, Error> where T: DeserializeOwned + Serialize + Default, {
    let dir = get_base_dir(app, None, None);
    fs::create_dir_all(&dir)?;
    let file_path = dir.join(&file_name);
    if !fs::exists(&file_path)? {
        fs::write(&file_path, serde_json::to_string_pretty(&T::default())?)?;
    }
    let contents = fs::read(&file_path)?;
    let processed_data = serde_json::from_slice(&contents)?;

    Ok(processed_data)
}

pub fn save_full_stats<T: Serialize>(app: &tauri::AppHandle, file_name: &str, data: &T) -> Result<(), Error> {
    let dir = get_base_dir(app, None, None);
    let file_path = dir.join(&file_name);
    let json = serde_json::to_string_pretty(&data)?;
    fs::write(file_path, json)?;

    Ok(())
}
