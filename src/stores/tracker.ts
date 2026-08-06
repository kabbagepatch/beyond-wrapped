import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia'
import { ref } from 'vue'

export type PlayCount = {
  primary: string;
  secondary?: string;
  playCount: number;
  msPlayed: number;
}
type TrackCounts = {
  tracks?: PlayCount[],
  artists?: PlayCount[],
  albums?: PlayCount[]
};
type YearlyTrackCount = { [year: number]: TrackCounts; };
type MonthlyTrackCount = { [year: number]: { [month: string]: TrackCounts } };

export type Play = {
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

export const useTrackerStore = defineStore('tracker', () => {
  const yearlyTotals = ref<YearlyTrackCount>({});
  const monthlyTotals = ref<MonthlyTrackCount>({});

  const getTopTracks = async (year: number, month?: number): Promise<PlayCount[]> => {
    return getTopItems('tracks', year, month);
  }

  const getTopArtists = async (year: number, month?: number): Promise<PlayCount[]> => {
    return getTopItems('artists', year, month);
  }

  const getTopAlbums = async (year: number, month?: number): Promise<PlayCount[]> => {
    return getTopItems('albums', year, month);
  }

  const playCountMap = (result: any): PlayCount[] => (
    result.map((i: any) : PlayCount => ({
      primary: i.primary,
      secondary: i.secondary,
      playCount: i.play_count,
      msPlayed: i.ms_played,
    }))
  );

  const getTopItems = async (item : 'tracks' | 'artists' | 'albums', year: number, month?: number): Promise<PlayCount[]> => {
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

  const getTopItemsCustom = async (item : 'tracks' | 'artists' | 'albums', from : string, to : string): Promise<PlayCount[]> => {
    const fromParts = from.split('-');
    const fromMonth = parseInt(fromParts[0], 10);
    const fromYear = parseInt(fromParts[1], 10);
    const toParts = to.split('-');
    const toMonth = parseInt(toParts[0], 10);
    const toYear = parseInt(toParts[1], 10);
    const result: any = await invoke('get_top_items_custom', { item, fromYear, fromMonth, toYear, toMonth });

    return playCountMap(result)
  }

  const playMap = (result: any): Play[] => (
    result.map((i: any) : Play => ({
      trackName: i.track,
      artistName: i.artist,
      albumName: i.album,
      timeStamp: i.time_stamp,
      msPlayed: i.ms_played,
    }))
  );

  const getTrackPlays = async (track: string, artist: string): Promise<Play[]> => {
    const result: any = await invoke('get_track_plays_track',  { track, artist });

    return playMap(result);
  }

  const getArtistPlays = async (artist: string): Promise<Play[]> => {
    const result: any = await invoke('get_track_plays_artist',  { artist });

    return playMap(result);
  }

  const getAlbumPlays = async (album: string, artist: string): Promise<Play[]> => {
    const result: any = await invoke('get_track_plays_album',  { album, artist });

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
    const result: any = await invoke('get_track_stats_artist',  { artist });

    return statsMap(result);
  }

  const getAlbumStats = async(album: string, artist: string): Promise<TrackStats[]> => {
    const result: any = await invoke('get_track_stats_album',  { album, artist });

    return statsMap(result);
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
  }
});
