import { BaseDirectory, readTextFile } from '@tauri-apps/plugin-fs';
import { defineStore } from 'pinia'
import { Ref, ref } from 'vue'

export interface TrackTotals { msPlayed: number; playCount: number; }
export type TotalsList = [ string, TrackTotals ][];
export interface YearlyTotals { [year: string]: TotalsList; }
export interface MonthlyTotals { [year: string]: { [month: string]: TotalsList }; }

export interface TrackData {
  id: string;
  key?: string;
  trackName: string;
  artistName: string;
  albumName: string;
  playCount?: number;
  firstPlayed?: string;
}
export interface Entry {
  msPlayed: number;
  timePlayed?: string;
  timeStamp: string;
  dateString?: string;
  timeString?: string;
}
export interface TrackStats {
  [key: string]: {
    info: TrackData,
    plays: Entry[],
    computed?: boolean,
    firstPlayed?: string,
  }
}
export interface ArtistStats { 
  [artistName: string]: {
    name: string,
    tracks: TrackStats,
    computed?: boolean,
    topTracks: TrackData[],
    firstTracks: TrackData[],
    totalPlays: number,
    timePlayed?: string,
  }
};
export interface AlbumStats { 
  [key: string]: {
    name: string,
    artistName: string,
    tracks: TrackStats,
    computed?: boolean,
    topTracks: TrackData[],
    firstTracks: TrackData[],
    totalPlays: number,
    timePlayed?: string,
  }
};

const createTopItemLoader = (fileName: string, yearlyCache: Ref<YearlyTotals>, monthlyCache: Ref<MonthlyTotals>) => {
  return async (year: string, month?: string) => {
    try {
      if (month) {
        if (monthlyCache.value[year]?.[month]) {
          return monthlyCache.value[year][month];
        }
        const module = await readTextFile(`processed_history/${year}/${month}/${fileName}`, { baseDir: BaseDirectory.AppData })
        monthlyCache.value[year] ??= {};
        monthlyCache.value[year][month] = JSON.parse(module).map(trasformEntry);
        return monthlyCache.value[year][month];
      }
      
      if (yearlyCache.value[year]) {
        return yearlyCache.value[year];
      }
      const module = await readTextFile(`processed_history/${year}/${fileName}`, { baseDir: BaseDirectory.AppData });
      yearlyCache.value[year] = JSON.parse(module).map(trasformEntry) as TotalsList;
      return yearlyCache.value[year];
    } catch (error) {
      console.log(`Error loading ${fileName} for ${year}${month ? `/${month}` : ''}`);
      console.log(error);
      return [];
    }
  }
}

const createFullStatsLoader = (fileName: string, statsCache: Ref<ArtistStats | AlbumStats>, createBaseEntry: (key: string) => any) => {
  return async (key: string) => {
    if (!statsCache.value || Object.keys(statsCache.value).length === 0) {
      const module = await readTextFile(`processed_history/${fileName}`, { baseDir: BaseDirectory.AppData });
      const fullStats = JSON.parse(module) as ({ [artist: string]: TrackStats });
      if (!fullStats[key]) return;

      Object.keys(fullStats).forEach(entryKey  => {
        statsCache.value[entryKey] ??= createBaseEntry(entryKey);
        statsCache.value[entryKey].tracks = fullStats[entryKey] as TrackStats;
      });
    }

    const stats = statsCache.value[key];
    if (stats.computed) {
      return stats;
    }
    console.log(`Computing stats for: ${key}`);
    Object.keys(stats.tracks).forEach(key => {
      stats.tracks[key] = trasformFullStats(stats.tracks[key]);
    });
    Object.entries(stats.tracks).sort((a, b) => b[1].plays.length - a[1].plays.length).forEach(([key, entry]) => {
      stats.topTracks.push({ ...entry.info, key, playCount: entry.plays.length });
    });
  
    let firstTimestamps : { [key : string]: number } = {};
    stats.totalPlays = 0;
    let totalMsPlayed = 0;
    Object.keys(stats.tracks).forEach(key => {
      firstTimestamps[key] = stats.tracks[key].plays.map(play => new Date(play.timeStamp).getTime()).sort((a, b) => a - b)[0];
      stats.totalPlays += stats.tracks[key].plays.length;
      totalMsPlayed += stats.tracks[key].plays.reduce((sum, play) => sum + play.msPlayed, 0);
    });
    const totalHours = Math.floor(totalMsPlayed / 3600000);
    const totalMinutes = Math.floor((totalMsPlayed % 3600000) / 60000);
    const totalSeconds = Math.floor((totalMsPlayed % 60000) / 1000);
    stats.timePlayed = `${totalHours}h ${totalMinutes}m ${totalSeconds}s`;

    const dateOptions = { year: 'numeric', month: '2-digit', day: '2-digit' } as const;
    Object.entries(firstTimestamps).sort((a, b) => (a[1] - b[1])).forEach(([key, timeStamp]) => {
      const date = new Date(timeStamp);
      stats.firstTracks.push({ ...stats.tracks[key].info, key, firstPlayed: date.toLocaleDateString('en-US', dateOptions) });
    });
    stats.computed = true;

    return stats;
  }
}

const trasformEntry = (item: any) => {
  return [item[0], { "msPlayed": item[1].ms_played, "playCount": item[1].play_count, }];
}

const trasformFullStats = (item: any) => {
  return {
    info: {
      id: item.info.id,
      trackName: item.info.track_name,
      artistName: item.info.artist_name,
      albumName: item.info.album_name,
    },
    plays: item.plays.map((i: any) => ({
      msPlayed: i.ms_played,
      timeStamp: i.time_stamp,
    })),
  };
}

export const useTrackerStore = defineStore('tracker', () => {
  const yearlyTopTracks = ref<YearlyTotals>({});
  const monthlyTopTracks = ref<MonthlyTotals>({});
  const yearlyTopArtists = ref<YearlyTotals>({});
  const monthlyTopArtists = ref<MonthlyTotals>({});
  const yearlyTopAlbums = ref<YearlyTotals>({});
  const monthlyTopAlbums = ref<MonthlyTotals>({});
  const yearlyTopPerDays = ref<YearlyTotals>({});
  const monthlyTopPerDays = ref<MonthlyTotals>({});
  const fullTrackStats = ref<TrackStats>({});
  const fullArtistStats = ref<ArtistStats>({});
  const fullAlbumStats = ref<AlbumStats>({});

  const getTopTracks = createTopItemLoader('top_tracks.json', yearlyTopTracks, monthlyTopTracks);
  const getTopArtists = createTopItemLoader('top_artists.json', yearlyTopArtists, monthlyTopArtists);
  const getTopAlbums = createTopItemLoader('top_albums.json', yearlyTopAlbums, monthlyTopAlbums);
  const getTopDays = createTopItemLoader('top_days.json', yearlyTopPerDays, monthlyTopPerDays);

  const getTopItems = async (type : 'tracks' | 'artists' | 'albums', from : string, to : string) => {
    const fromParts = from.split('-');
    const fromMonth = parseInt(fromParts[0], 10);
    const fromYear = parseInt(fromParts[1], 10);
    const toParts = to.split('-');
    const toMonth = parseInt(toParts[0], 10);
    const toYear = parseInt(toParts[1], 10);
    const combinedTotals: TotalsList = [];

    for (let year = fromYear; year <= toYear; year += 1) {
      if (year > fromYear && year < toYear) {
        let yearlyData;
        switch(type) {
          case 'artists': yearlyData = await getTopArtists(year.toString()); break;
          case 'albums': yearlyData = await getTopAlbums(year.toString()); break;
          default: yearlyData = await getTopTracks(year.toString()); break;
        }
        yearlyData.forEach(([trackKey, totals]) => {
          const existingEntry = combinedTotals.find(entry => entry[0] === trackKey);
          if (existingEntry) {
            existingEntry[1].msPlayed += totals.msPlayed;
            existingEntry[1].playCount += totals.playCount;
          } else {
            combinedTotals.push([trackKey, { msPlayed: totals.msPlayed, playCount: totals.playCount }]);
          }
        });
        continue;
      }

      const startMonth = (year === fromYear) ? fromMonth : 1;
      const endMonth = (year === toYear) ? toMonth : 12;
      for (let month = startMonth; month <= endMonth; month += 1) {
        let monthlyData;
        switch(type) {
          case 'artists': monthlyData = await getTopArtists(year.toString(), month.toString()); break;
          case 'albums': monthlyData = await getTopAlbums(year.toString(), month.toString()); break;
          default: monthlyData = await getTopTracks(year.toString(), month.toString()); break;
        }
        monthlyData.forEach(([trackKey, totals]) => {
          const existingEntry = combinedTotals.find(entry => entry[0] === trackKey);
          if (existingEntry) {
            existingEntry[1].msPlayed += totals.msPlayed;
            existingEntry[1].playCount += totals.playCount;
          } else {
            combinedTotals.push([trackKey, { msPlayed: totals.msPlayed, playCount: totals.playCount }]);
          }
        });
      }
    }

    combinedTotals.sort((a, b) => b[1].playCount - a[1].playCount);
    return combinedTotals;
  }

  const getTrackStats = async (trackKey: string) => {
    if (!fullTrackStats.value || Object.keys(fullTrackStats.value).length === 0) {
      const module = await readTextFile(`processed_history/full_track_stats.json`, { baseDir: BaseDirectory.AppData });
      fullTrackStats.value = JSON.parse(module) as TrackStats;
    }

    if (!fullTrackStats.value[trackKey]) {
      return;
    }
    if (fullTrackStats.value[trackKey].computed) {
      return fullTrackStats.value[trackKey];
    }

    const dateOptions = { year: 'numeric', month: '2-digit', day: '2-digit' } as const;
    const timeOptions = { hour: 'numeric', minute: '2-digit' } as const;
    fullTrackStats.value[trackKey] = trasformFullStats(fullTrackStats.value[trackKey]);
    fullTrackStats.value[trackKey].plays = fullTrackStats.value[trackKey].plays.map(play => {
      const date = new Date(play.timeStamp);
      
      const minutesPlayed = Math.floor(play.msPlayed / 60000);
      const secondsPlayed = Math.floor((play.msPlayed % 60000) / 1000);

      return { 
        ...play,
        dateString: date.toLocaleDateString('en-US', dateOptions),
        timeString: date.toLocaleTimeString('en-US', timeOptions),
        timePlayed: `${minutesPlayed}m ${secondsPlayed}s`,
      };
    }).reverse();
    const firstTimestamp = fullTrackStats.value[trackKey].plays[fullTrackStats.value[trackKey].plays.length - 1].timeStamp;
    fullTrackStats.value[trackKey].firstPlayed = new Date(firstTimestamp).toLocaleDateString('en-US', dateOptions);
    fullTrackStats.value[trackKey].computed = true;
    return fullTrackStats.value[trackKey];
  }

  const getArtistStats = createFullStatsLoader(
    "full_artist_stats.json",
    fullArtistStats,
    artist => ({ name: artist, tracks: {}, totalPlays: 0, topTracks: [], firstTracks: []  }),
  );

  const getAlbumStats = createFullStatsLoader(
    "full_album_stats.json",
    fullAlbumStats,
    album => ({ name: album.split(' - ')[0], artistName: album.split(' - ')[1], tracks: {}, totalPlays: 0, topTracks: [], firstTracks: []  }),
  );

  return {
    getTopTracks,
    getTopItems,
    getTopArtists,
    getTopAlbums,
    getTopDays,
    getTrackStats,
    getArtistStats,
    getAlbumStats,
  }
});
