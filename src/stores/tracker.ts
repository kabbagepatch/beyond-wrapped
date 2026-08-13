import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia'
import { ref } from 'vue'

export type ItemPlayData = {
  primary: string;
  secondary?: string;
  playCount: number;
  msPlayed: number;
}
type TrackCounts = {
  tracks?: ItemPlayData[],
  artists?: ItemPlayData[],
  albums?: ItemPlayData[]
};
type YearlyTrackCount = { [year: number]: TrackCounts; };
type MonthlyTrackCount = { [year: number]: { [month: string]: TrackCounts } };

export type PlayCounts = { label: string, count: number }[];
type YearlyPlayCount = { [year: number]: PlayCounts; };
type MonthlyPlayCount = { [year: number]: { [month: string]: PlayCounts } };

export type PlayEntry = {
  trackName: string;
  artistName: string;
  albumName: string;
  timeStamp: string;
  msPlayed: number;
}

export type TrackStats = {
  trackName: string;
  artistName: string;
  albumName: string;
  playCount: number;
  msPlayed: number;
  firstPlay: string;
}

export type Track = { name: string; artist: string; album: string; playCount: number };
export type Artist = { name: string; playCount: number };
export type Album = { name: string; artist: string; playCount: number };
export type SearchResults = {
  tracks: Track[];
  artists: Artist[];
  albums: Album[];
}

export const useTrackerStore = defineStore('tracker', () => {
  const yearlyTotals = ref<YearlyTrackCount>({});
  const monthlyTotals = ref<MonthlyTrackCount>({});
  const monthlyCounts = ref<YearlyPlayCount>({});
  const dailyCounts = ref<MonthlyPlayCount>({});

  const getTopTracks = async (year: number, month?: number): Promise<ItemPlayData[]> => {
    return getTopItems('tracks', year, month);
  }

  const getTopArtists = async (year: number, month?: number): Promise<ItemPlayData[]> => {
    return getTopItems('artists', year, month);
  }

  const getTopAlbums = async (year: number, month?: number): Promise<ItemPlayData[]> => {
    return getTopItems('albums', year, month);
  }

  const playCountMap = (result: any): ItemPlayData[] => (
    result.map((i: any) : ItemPlayData => ({
      primary: i.primary,
      secondary: i.secondary,
      playCount: i.play_count,
      msPlayed: i.ms_played,
    }))
  );

  const getTopItems = async (item : 'tracks' | 'artists' | 'albums', year: number, month?: number): Promise<ItemPlayData[]> => {
    if (month) {
      if (monthlyTotals.value[year]?.[month]?.[item]?.length) {
        return monthlyTotals.value[year][month][item];
      }
    } else {
      if (yearlyTotals.value[year]?.[item]?.length) {
        return yearlyTotals.value[year][item];
      }
    }

    const result: any = await invoke('get_top_items', { item, year, month });

    const mapped = playCountMap(result)

    if (month) {
      if (!monthlyTotals.value[year]) monthlyTotals.value[year] = {};
      if (!monthlyTotals.value[year][month]) monthlyTotals.value[year][month] = {};
      monthlyTotals.value[year][month][item] = mapped;
    } else {
      if (!yearlyTotals.value[year]) yearlyTotals.value[year] = {};
      yearlyTotals.value[year][item] = mapped;
    }

    return mapped;
  }

  const getTopItemsCustom = async (item : 'tracks' | 'artists' | 'albums', from : string, to : string): Promise<ItemPlayData[]> => {
    const fromParts = from.split('-');
    const fromMonth = parseInt(fromParts[0], 10);
    const fromYear = parseInt(fromParts[1], 10);
    const toParts = to.split('-');
    const toMonth = parseInt(toParts[0], 10);
    const toYear = parseInt(toParts[1], 10);
    const result: any = await invoke('get_top_items_custom', { item, fromYear, fromMonth, toYear, toMonth });

    return playCountMap(result)
  }

  const playMap = (result: any): PlayEntry[] => (
    result.map((i: any) : PlayEntry => ({
      trackName: i.track,
      artistName: i.artist,
      albumName: i.album,
      timeStamp: i.time_stamp,
      msPlayed: i.ms_played,
    }))
  );

  const getTrackPlays = async (track: string, artist: string): Promise<PlayEntry[]> => {
    const result: any = await invoke('get_track_plays_track', { track, artist });

    return playMap(result);
  }

  const getArtistPlays = async (artist: string): Promise<PlayEntry[]> => {
    const result: any = await invoke('get_track_plays_artist', { artist });

    return playMap(result);
  }

  const getAlbumPlays = async (album: string, artist: string): Promise<PlayEntry[]> => {
    const result: any = await invoke('get_track_plays_album', { album, artist });

    return playMap(result);
  }

  const statsMap = (result: any): TrackStats[] => (
    result.map((i: any) : TrackStats => ({
      trackName: i.track,
      artistName: i.artist,
      albumName: i.album,
      playCount: i.play_count,
      msPlayed: i.ms_played,
      firstPlay: i.first_play,
    }))
  );

  const getArtistStats = async(artist: string): Promise<TrackStats[]> => {
    const result: any = await invoke('get_track_stats_artist', { artist });

    return statsMap(result);
  }

  const getAlbumStats = async(album: string, artist: string): Promise<TrackStats[]> => {
    const result: any = await invoke('get_track_stats_album', { album, artist });

    return statsMap(result);
  }

  const searchItems = async(searchString: string): Promise<SearchResults> => {
    const result: any = await invoke('search_items', { searchString });

    return result;
  }

  const getMonthlyCounts = async(year: number): Promise<PlayCounts> => {
    if (monthlyCounts.value[year]?.length) {
      return monthlyCounts.value[year];
    }
    const result: any = await invoke('get_monthly_play_counts', { year });

    const months = ['J', 'F', 'M', 'A', 'M', 'J', 'J', 'A', 'S', 'O', 'N', 'D'];
    const mapped = months.map((label, index) => {
      const month = index + 1;
      const found = result.find(
        (r: any) => parseInt(r.primary.split('-')[1], 10) === month
      );

      return {
        label,
        count: found?.play_count ?? 0,
      };
    });

    monthlyCounts.value[year] = mapped;

    return mapped;
  }

  const getDailyCounts = async(year: number, month: number): Promise<PlayCounts> => {
    if (dailyCounts.value[year]?.[month]?.length) {
      return dailyCounts.value[year][month];
    }

    const result: any = await invoke('get_daily_play_counts', { year, month });

    const mapped = result.map((r: any) => {
      return {
        label: r.primary.split('-')[2],
        count: r.play_count,
      };
    });

    if (!dailyCounts.value[year]) dailyCounts.value[year] = {};
    dailyCounts.value[year][month] = mapped;

    return mapped;
  }

  const yearlyCountMap = (result: any) => {
    return result.map((r: any) => {
      return {
        label: r.primary,
        count: r.play_count,
      };
    });
  }

  const getCustomCounts = async(from: string, to: string): Promise<PlayCounts> => {
    const fromParts = from.split('-');
    const fromMonth = parseInt(fromParts[0], 10);
    const fromYear = parseInt(fromParts[1], 10);
    const toParts = to.split('-');
    const toMonth = parseInt(toParts[0], 10);
    const toYear = parseInt(toParts[1], 10);

    if (fromYear === toYear) {
      const months = ['J', 'F', 'M', 'A', 'M', 'J', 'J', 'A', 'S', 'O', 'N', 'D'];
      const result: any = await invoke('get_play_counts', { freq: 'month', fromYear, fromMonth, toYear, toMonth });
      return result.map((r: any) => {
        return {
          label: months[parseInt(r.primary.split('-')[1], 10) - 1],
          count: r.play_count,
        };
      });
    } else {
      const result: any = await invoke('get_play_counts', { freq: 'year', fromYear, fromMonth, toYear, toMonth });

      return yearlyCountMap(result);
    }
  }

  const getTrackPlayCounts = async(track: string, artist: string): Promise<PlayCounts> => {
    const result: any = await invoke('get_item_play_counts', { freq: "year", artist, track });

    return yearlyCountMap(result);
  }

  const getArtistPlayCounts = async(artist: string): Promise<PlayCounts> => {
    const result: any = await invoke('get_item_play_counts', { freq: "year", artist });

    return yearlyCountMap(result);
  }

  const getAlbumPlayCounts = async(album: string, artist: string): Promise<PlayCounts> => {
    const result: any = await invoke('get_item_play_counts', { freq: "year", artist, album });

    return yearlyCountMap(result);
  }

  return {
    getTopTracks,
    getTopArtists,
    getTopAlbums,
    getTopItemsCustom,
    getTrackPlays,
    getArtistPlays,
    getArtistStats,
    getAlbumPlays,
    getAlbumStats,
    searchItems,
    getMonthlyCounts,
    getDailyCounts,
    getCustomCounts,
    getTrackPlayCounts,
    getArtistPlayCounts,
    getAlbumPlayCounts,
  }
});
