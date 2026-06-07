use std::{collections::HashMap, path::PathBuf};

use chrono::{DateTime, Datelike, Local};
use tauri_plugin_store::StoreExt;

use crate::{
    AppError, 
    file::{
        create_stats_dir, get_full_stats, get_months_dir_names, get_raw_track_data, get_saved_processed_data, get_years_dir_names, rename_temp_file, save_full_stats, save_processed_data
    },
    models::{Entry, EntryStats, FullAlbumStats, FullArtistStats, FullTrackStats, MonthlyStats, TrackPlay, TrackStats, YearlyStats}
};

pub fn process_raw_history_files(app: &tauri::AppHandle, raw_json_files: &Vec<PathBuf>) -> Result<(), AppError> {
    let store = app.store("store.json")?;
    let mut full_track_stats: FullTrackStats = get_full_stats(app, "full_track_stats.json")?;
    let mut full_artist_stats: FullArtistStats = get_full_stats(app, "full_artist_stats.json")?;
    let mut full_album_stats: FullAlbumStats = get_full_stats(app, "full_album_stats.json")?;

    for raw_entry in raw_json_files {
        let file_name = raw_entry.file_name().unwrap().to_string_lossy();
        let status: String = store.get(file_name.to_lowercase()).and_then(|v| v.as_str().map(str::to_owned)).unwrap_or("unknown".to_owned());
        if status == "processed" {
            continue;
        }
        store.set(file_name.to_lowercase(), "processing");
        println!("Processing {}", file_name);

        let track_data = match get_raw_track_data(raw_entry.as_path()) {
            Ok(c) => c,
            Err(err) => {
                store.set(file_name.to_lowercase(), "error");
                eprintln!("Failed to read {}: {}", file_name, err);
                continue;
            }
        };

        let mut yearly_track_stats: YearlyStats = YearlyStats::new();
        let mut yearly_artist_stats: YearlyStats = YearlyStats::new();
        let mut yearly_album_stats: YearlyStats = YearlyStats::new();
        let mut yearly_per_day_stats: YearlyStats = YearlyStats::new();
        let mut yearly_per_month_stats: MonthlyStats = MonthlyStats::new();
        for entry in track_data {
            if entry.ms_played < 30000 {
                continue;
            }

            let date = DateTime::parse_from_rfc3339(&entry.time_stamp)?.with_timezone(&Local);
            let year = date.year();
            let month = date.month();
            let day = date.day();

            /* Track Stats */
            let track_stats: &mut EntryStats = yearly_track_stats
                .entry(year)
                .or_insert_with(MonthlyStats::new)
                .entry(month)
                .or_insert_with(|| {
                    create_stats_dir(&app, year, Some(month)).unwrap();

                    get_saved_processed_data(&app, year, Some(month), "track_stats.json").unwrap_or_else(|_| EntryStats::new())
                });
            let track_key = format!("{} - {}", entry.track.track_name, entry.track.artist_name);
            let track_entry = track_stats.entry(track_key.clone()).or_insert(Entry { ms_played: 0, play_count: 0 });
            track_entry.play_count += 1;
            track_entry.ms_played += entry.ms_played;

            let full_track_stat: &mut TrackStats = full_track_stats
                .entry(track_key.clone())
                .or_insert(TrackStats { info: entry.track.clone(), plays: Vec::new() });
            full_track_stat.plays.push(TrackPlay { time_stamp: entry.time_stamp.clone(), ms_played: entry.ms_played });

            /* Artist Stats */
            let artist_stats: &mut EntryStats = yearly_artist_stats
                .entry(year)
                .or_insert_with(MonthlyStats::new)
                .entry(month)
                .or_insert_with(|| {
                    get_saved_processed_data(&app, year, Some(month), "artist_stats.json").unwrap_or_else(|_| EntryStats::new())
                });
            let artist_key = format!("{}", entry.track.clone().artist_name);
            let artist_entry = artist_stats.entry(artist_key.clone()).or_insert(Entry { ms_played: 0, play_count: 0 });
            artist_entry.play_count += 1;
            artist_entry.ms_played += entry.ms_played;

            let full_artist_stat: &mut TrackStats = full_artist_stats
                .entry(artist_key)
                .or_insert_with(HashMap::new)
                .entry(track_key.clone())
                .or_insert(TrackStats { info: entry.track.clone(), plays: Vec::new() });
            full_artist_stat.plays.push(TrackPlay { time_stamp: entry.time_stamp.clone(), ms_played: entry.ms_played });

            /* Album Stats */
            let album_stats: &mut EntryStats = yearly_album_stats
                .entry(year)
                .or_insert_with(MonthlyStats::new)
                .entry(month)
                .or_insert_with(|| {
                    get_saved_processed_data(&app, year, Some(month), "album_stats.json").unwrap_or_else(|_| EntryStats::new())
                });
            let album_key = format!("{} - {}", entry.track.album_name, entry.track.artist_name);
            let album_entry = album_stats.entry(album_key.clone()).or_insert(Entry { ms_played: 0, play_count: 0 });
            album_entry.play_count += 1;
            album_entry.ms_played += entry.ms_played;

            let full_album_stat: &mut TrackStats = full_album_stats
                .entry(album_key)
                .or_insert_with(HashMap::new)
                .entry(track_key)
                .or_insert(TrackStats { info: entry.track, plays: Vec::new() });
            full_album_stat.plays.push(TrackPlay { time_stamp: entry.time_stamp, ms_played: entry.ms_played });
            
            /* Daily/Monthly Stats */
            let per_day_stats: &mut EntryStats = yearly_per_day_stats
                .entry(year)
                .or_insert_with(MonthlyStats::new)
                .entry(month)
                .or_insert_with(|| {
                    get_saved_processed_data(&app, year, Some(month), "per_day_stats.json").unwrap_or_else(|_| EntryStats::new())
                });
            let per_day_entry = per_day_stats.entry(day.to_string()).or_insert(Entry { ms_played: 0, play_count: 0 });
            per_day_entry.play_count += 1;
            per_day_entry.ms_played += entry.ms_played;
            
            let per_month_stats: &mut EntryStats = yearly_per_month_stats
                .entry(year as u32)
                .or_insert_with(|| {
                    get_saved_processed_data(&app, year, None, "per_month_stats.json").unwrap_or_else(|_| EntryStats::new())
                });
            let per_month_entry = per_month_stats.entry(month.to_string()).or_insert(Entry { ms_played: 0, play_count: 0 });
            per_month_entry.play_count += 1;
            per_month_entry.ms_played += entry.ms_played;
        }

        for (yearly_stats, file_name) in [
            (&yearly_track_stats, "track_stats.json.tmp"),
            (&yearly_artist_stats, "artist_stats.json.tmp"),
            (&yearly_album_stats, "album_stats.json.tmp"),
            (&yearly_per_day_stats, "per_day_stats.json.tmp"),
        ] {
            for (year, monthly_stats) in yearly_stats {
                for (month, stats) in monthly_stats {
                    save_processed_data(&app, *year, Some(*month), file_name, stats);
                }
            }
        }
        for (year, per_month_stats) in yearly_per_month_stats {
            save_processed_data(&app, year as i32, None, "per_month_stats.json.tmp", &per_month_stats);
        }
        save_full_stats(app, "full_track_stats.json.tmp", &full_track_stats)?;
        save_full_stats(app, "full_artist_stats.json.tmp", &full_artist_stats)?;
        save_full_stats(app, "full_album_stats.json.tmp", &full_album_stats)?;

        for (year, monthly_stats) in &yearly_track_stats {
            for (month, _) in monthly_stats {
                rename_temp_file(&app, Some(*year), Some(*month), "track_stats.json");
                rename_temp_file(&app, Some(*year), Some(*month), "artist_stats.json");
                rename_temp_file(&app, Some(*year), Some(*month), "album_stats.json");
                rename_temp_file(&app, Some(*year), Some(*month), "per_day_stats.json");
            }
            store.set(format!("{}-top-items", year), "reset");
            rename_temp_file(&app, Some(*year), None, "per_month_stats.json");
        }
        rename_temp_file(&app, None, None, "full_track_stats.json");
        rename_temp_file(&app, None, None, "full_artist_stats.json");
        rename_temp_file(&app, None, None, "full_album_stats.json");

        store.set(file_name.to_lowercase(), "processed");
        store.save()?;
    }

    Ok(())
}

pub fn process_top_items(app: &tauri::AppHandle) -> Result<(), AppError> {
    let store = app.store("store.json")?;

    for year in get_years_dir_names(app)? {
        let status: String = store.get(format!("{}-top-items", year)).and_then(|v| v.as_str().map(str::to_owned)).unwrap_or("unknown".to_owned());
        if status == "processed" {
            continue;
        }
        store.set(format!("{}-top-items", year), "processing");

        for item in ["track", "artist", "album", "per_day"] {
            let mut yearly_aggregate: HashMap<String, Entry> = HashMap::new();
            let month_dirs = match get_months_dir_names(app, year) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Unable to get months for {}: {}", year, e);
                    continue;
                }
            };
            for month in month_dirs {
                let mut montly_aggregate: HashMap<String, Entry> = HashMap::new();
                let item_stats = match get_saved_processed_data(&app, year, Some(month), &format!("{}_stats.json", item)) {
                    Ok(c) => c,
                    Err(_) => { continue; }
                };
                for (key, entry) in item_stats {
                    let existing_entry = yearly_aggregate.entry(key.clone()).or_insert(Entry { ms_played: 0, play_count: 0 });
                    existing_entry.play_count += entry.play_count;
                    existing_entry.ms_played += entry.ms_played;
                    
                    let existing_entry = montly_aggregate.entry(key).or_insert(Entry { ms_played: 0, play_count: 0 });
                    existing_entry.play_count += entry.play_count;
                    existing_entry.ms_played += entry.ms_played;
                }
                let mut sorted_items: Vec<(&String, &Entry)> = montly_aggregate.iter().collect();
                sorted_items.sort_by(|a, b| {
                    b.1.play_count.cmp(&a.1.play_count)
                });
                save_processed_data(app, year, Some(month), &format!("top_{}s.json.tmp", item), &sorted_items);
            }
            if item != "per_day" {
                let mut sorted_items: Vec<(&String, &Entry)> = yearly_aggregate.iter().collect();
                sorted_items.sort_by(|a, b| {
                    b.1.play_count.cmp(&a.1.play_count)
                });
                save_processed_data(app, year, None, &format!("top_{}s.json.tmp", item), &sorted_items);
            }
        }

        for item in ["track", "artist", "album", "per_day"] {
            let month_dirs = match get_months_dir_names(app, year) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Unable to get months for {}: {}", year, e);
                    continue;
                }
            };
            for month in month_dirs {
                rename_temp_file(&app, Some(year), Some(month), &format!("top_{}s.json", item));
            }
            if item != "per_day" {
                rename_temp_file(&app, Some(year), None, &format!("top_{}s.json", item));
            }
        }

        println!("Saved top items for {}", year);
        store.set(format!("{}-top-items", year), "processed");
    }
    Ok(())
}
