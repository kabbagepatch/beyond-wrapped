import { BaseDirectory, DirEntry, exists, mkdir, readDir, readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
import { load } from "@tauri-apps/plugin-store";

type SongData = {
  id: string;
  trackName: string;
  artistName: string;
  albumName: string;
}

type Entry = { msPlayed: number; playCount: number; }

type SongEntryData = {
  song: SongData;
  timeStamp: string;
  msPlayed: number;
}

type YearlyStats = {
  [year: string]: {
    [month: number]: {
      [key: string]: Entry;
    }
  }
};

type SongStats = {
  [key: string]: {
    info: SongData,
    plays: {
      timeStamp: string;
      msPlayed: number;
    }[]
  }
}
type ArtistStats = { [artistName: string]: SongStats };

const EXTENDED_HISTORY_PATH = 'rawExtendedHistory';
const PROCESSSED_DATA_PATH = 'processedHistory';
const FULL_SONGS_STATS_PATH = 'processedHistory/fullSongStats.json';
const FULL_ARTISTS_STATS_PATH = 'processedHistory/fullArtistStats.json';
const FULL_ALBUM_STATS_PATH = 'processedHistory/fullAlbumStats.json';
const YEARLY_DATA_PATH = (year : string | number) => `${PROCESSSED_DATA_PATH}/yearly/${year}`;
const MONTHLY_DATA_PATH = (year : string | number, month: string | number) => `${YEARLY_DATA_PATH(year)}/${month}`;
const SONG_STATS_PATH = (year : string | number, month: string | number) => `${MONTHLY_DATA_PATH(year, month)}/songStats.json`;
const ARTIST_STATS_PATH = (year : string | number, month: string | number) => `${MONTHLY_DATA_PATH(year, month)}/artistStats.json`;
const ALBUM_STATS_PATH = (year : string | number, month: string | number) => `${MONTHLY_DATA_PATH(year, month)}/albumStats.json`;

export const processExtendedData = async () => {
  let songStats = {} as SongStats;
  let artistStats = {} as ArtistStats;
  let albumStats = {} as ArtistStats;
  const yearlySongStats = {} as YearlyStats;
  const yearlyArtistStats = {} as YearlyStats;
  const yearlyAlbumStats = {} as YearlyStats;

  const entries = await readDir(EXTENDED_HISTORY_PATH, { baseDir: BaseDirectory.AppData });
  for (const file of entries) {
    if (file.isFile && file.name.startsWith('Streaming_History_Audio_') && file.name.endsWith('.json')) {
      console.log(`Reading ${file.name}...`)
      const fileData = await readTextFile(`rawExtendedHistory/${file.name}`, { baseDir: BaseDirectory.AppData });
      const songData = JSON.parse(fileData).map((entry: any) : SongEntryData => {
        return {
          song: {
            id: entry.spotify_track_uri,
            trackName: entry.master_metadata_track_name,
            artistName: entry.master_metadata_album_artist_name,
            albumName: entry.master_metadata_album_album_name,
          },
          timeStamp: entry.ts,
          msPlayed: entry.ms_played,
        };
      }).filter((entry : SongEntryData) => entry.song.id);
      
      console.log(`Processing ${file.name}...`)
      songData.forEach((entry: SongEntryData) => {
        if (entry.msPlayed < 30000) return;
        const date = new Date(entry.timeStamp);
        const year = date.getFullYear().toString();
        let month = (date.getMonth() + 1);

        const songKey = `${entry.song.trackName} - ${entry.song.artistName}`;
        const albumKey = `${entry.song.albumName} - ${entry.song.artistName}`;

        if (!yearlySongStats[year]) yearlySongStats[year] = {};
        if (!yearlySongStats[year][month]) yearlySongStats[year][month] = {};
        if (!yearlySongStats[year][month][songKey]) yearlySongStats[year][month][songKey] = { msPlayed: 0, playCount: 0 };
        yearlySongStats[year][month][songKey].msPlayed += entry.msPlayed;
        yearlySongStats[year][month][songKey].playCount += 1;

        if (!yearlyArtistStats[year]) yearlyArtistStats[year] = {};
        if (!yearlyArtistStats[year][month]) yearlyArtistStats[year][month] = {};
        if (!yearlyArtistStats[year][month][entry.song.artistName]) yearlyArtistStats[year][month][entry.song.artistName] = { msPlayed: 0, playCount: 0 };
        yearlyArtistStats[year][month][entry.song.artistName].msPlayed += entry.msPlayed;
        yearlyArtistStats[year][month][entry.song.artistName].playCount += 1;

        if (!yearlyAlbumStats[year]) yearlyAlbumStats[year] = {};
        if (!yearlyAlbumStats[year][month]) yearlyAlbumStats[year][month] = {};
        if (!yearlyAlbumStats[year][month][albumKey]) yearlyAlbumStats[year][month][albumKey] = { msPlayed: 0, playCount: 0 };
        yearlyAlbumStats[year][month][albumKey].msPlayed += entry.msPlayed;
        yearlyAlbumStats[year][month][albumKey].playCount += 1;

        if (!songStats[songKey]) songStats[songKey] = { info: entry.song, plays: [] };
        songStats[songKey].plays.push({
          timeStamp: entry.timeStamp,
          msPlayed: entry.msPlayed,
        });

        if (!artistStats[entry.song.artistName]) artistStats[entry.song.artistName] = {};
        if (!artistStats[entry.song.artistName][songKey]) artistStats[entry.song.artistName][songKey] = { info: entry.song, plays: [] };
        artistStats[entry.song.artistName][songKey].plays.push({
          timeStamp: entry.timeStamp,
          msPlayed: entry.msPlayed,
        });

        if (!albumStats[albumKey]) albumStats[albumKey] = {};
        if (!albumStats[albumKey][songKey]) albumStats[albumKey][songKey] = { info: entry.song, plays: [] };
        albumStats[albumKey][songKey].plays.push({
          timeStamp: entry.timeStamp,
          msPlayed: entry.msPlayed,
        });
      });
      console.log(`Processed ${file.name}`)
    }
  }

  console.log('Saving Processed Data')
  const processHistoryExists = await exists(PROCESSSED_DATA_PATH, { baseDir: BaseDirectory.AppData });
  if (!processHistoryExists) await mkdir(PROCESSSED_DATA_PATH, { baseDir: BaseDirectory.AppData });
  const yearlyDirExists = await exists(PROCESSSED_DATA_PATH + '/yearly', { baseDir: BaseDirectory.AppData });
  if (!yearlyDirExists) await mkdir(PROCESSSED_DATA_PATH + '/yearly', { baseDir: BaseDirectory.AppData });
  for (const year in yearlySongStats) {
    const yearlyExists = await exists(YEARLY_DATA_PATH(year), { baseDir: BaseDirectory.AppData });
    if (!yearlyExists) await mkdir(YEARLY_DATA_PATH(year), { baseDir: BaseDirectory.AppData });
    for (const month in yearlySongStats[year]) {
      const monthlyExists = await exists(MONTHLY_DATA_PATH(year, month), { baseDir: BaseDirectory.AppData });
      if (!monthlyExists) await mkdir(MONTHLY_DATA_PATH(year, month), { baseDir: BaseDirectory.AppData });
      await writeTextFile(SONG_STATS_PATH(year, month), JSON.stringify(yearlySongStats[year][month], null, 2), { baseDir: BaseDirectory.AppData });
      await writeTextFile(ARTIST_STATS_PATH(year, month), JSON.stringify(yearlyArtistStats[year][month], null, 2), { baseDir: BaseDirectory.AppData });
      await writeTextFile(ALBUM_STATS_PATH(year, month), JSON.stringify(yearlyAlbumStats[year][month], null, 2), { baseDir: BaseDirectory.AppData });
    }
  }
  await writeTextFile(FULL_SONGS_STATS_PATH, JSON.stringify(songStats, null, 2), { baseDir: BaseDirectory.AppData });
  await writeTextFile(FULL_ARTISTS_STATS_PATH, JSON.stringify(artistStats, null, 2), { baseDir: BaseDirectory.AppData });
  await writeTextFile(FULL_ALBUM_STATS_PATH, JSON.stringify(albumStats, null, 2), { baseDir: BaseDirectory.AppData });
  console.log('Processed Data Successfully Saved');

  await saveTopItemsData()
}

const saveTopItemsData = async () => {
  const store = await load('store.json');
  console.log('Processing Top Items...');
  const yearFolders = (await readDir(`${PROCESSSED_DATA_PATH}/yearly/`, { baseDir: BaseDirectory.AppData })).filter((d:DirEntry) => d.isDirectory).map((d:DirEntry) => d.name);
  for (const year of yearFolders) {
    console.log(`Processing ${year}...`);
    const yearlySongAggregates: { [key: string]: Entry } = {};
    const yearlyArtistAggregates: { [key: string]: Entry } = {};
    const yearlyAlbumAggregates: { [key: string]: Entry } = {};

    const monthFolders = (await readDir(YEARLY_DATA_PATH(year), { baseDir: BaseDirectory.AppData })).filter((d:DirEntry) => d.isDirectory).map((d:DirEntry) => parseInt(d.name, 10));
    for (const month of monthFolders) {
      const monthlySongAggregates: { [key: string]: Entry } = {};
      const monthlyArtistAggregates: { [key: string]: Entry } = {};
      const monthlyAlbumAggregates: { [key: string]: Entry } = {};

      const songStats = JSON.parse(await readTextFile(SONG_STATS_PATH(year, month), { baseDir: BaseDirectory.AppData })) as YearlyStats[''][0];
      for (const key in songStats) {
        if (!yearlySongAggregates[key]) yearlySongAggregates[key] = { msPlayed: 0, playCount: 0 };
        yearlySongAggregates[key].msPlayed += songStats[key].msPlayed;
        yearlySongAggregates[key].playCount += songStats[key].playCount;
        if (!monthlySongAggregates[key]) monthlySongAggregates[key] = { msPlayed: 0, playCount: 0 };
        monthlySongAggregates[key].msPlayed += songStats[key].msPlayed;
        monthlySongAggregates[key].playCount += songStats[key].playCount;
      }

      const artistStats = JSON.parse(await readTextFile(ARTIST_STATS_PATH(year, month), { baseDir: BaseDirectory.AppData })) as YearlyStats[''][0];
      for (const key in artistStats) {
        if (!yearlyArtistAggregates[key]) yearlyArtistAggregates[key] = { msPlayed: 0, playCount: 0 };
        yearlyArtistAggregates[key].msPlayed += artistStats[key].msPlayed;
        yearlyArtistAggregates[key].playCount += artistStats[key].playCount;
        if (!monthlyArtistAggregates[key]) monthlyArtistAggregates[key] = { msPlayed: 0, playCount: 0 };
        monthlyArtistAggregates[key].msPlayed += artistStats[key].msPlayed;
        monthlyArtistAggregates[key].playCount += artistStats[key].playCount;
      }

      const albumStats = JSON.parse(await readTextFile(ALBUM_STATS_PATH(year, month), { baseDir: BaseDirectory.AppData })) as YearlyStats[''][0];
      for (const key in albumStats) {
        if (!yearlyAlbumAggregates[key]) yearlyAlbumAggregates[key] = { msPlayed: 0, playCount: 0 };
        yearlyAlbumAggregates[key].msPlayed += albumStats[key].msPlayed;
        yearlyAlbumAggregates[key].playCount += albumStats[key].playCount;
        if (!monthlyAlbumAggregates[key]) monthlyAlbumAggregates[key] = { msPlayed: 0, playCount: 0 };
        monthlyAlbumAggregates[key].msPlayed += albumStats[key].msPlayed;
        monthlyAlbumAggregates[key].playCount += albumStats[key].playCount;
      }

      let sortedSongs = Object.entries(monthlySongAggregates).sort((a, b) => b[1].playCount - a[1].playCount);
      await writeTextFile(`${MONTHLY_DATA_PATH(year, month)}/topSongs.json`, JSON.stringify(sortedSongs, null, 2), { baseDir: BaseDirectory.AppData });
      let sortedArtists = Object.entries(monthlyArtistAggregates).sort((a, b) => b[1].playCount - a[1].playCount);
      await writeTextFile(`${MONTHLY_DATA_PATH(year, month)}/topArtists.json`, JSON.stringify(sortedArtists, null, 2), { baseDir: BaseDirectory.AppData });
      let sortedAlbums = Object.entries(monthlyAlbumAggregates).sort((a, b) => b[1].playCount - a[1].playCount);
      await writeTextFile(`${MONTHLY_DATA_PATH(year, month)}/topAlbums.json`, JSON.stringify(sortedAlbums, null, 2), { baseDir: BaseDirectory.AppData });
    }

    let sortedSongs = Object.entries(yearlySongAggregates).sort((a, b) => b[1].playCount - a[1].playCount);
    await writeTextFile(`${YEARLY_DATA_PATH(year)}/topSongs.json`, JSON.stringify(sortedSongs, null, 2), { baseDir: BaseDirectory.AppData });
    let sortedArtists = Object.entries(yearlyArtistAggregates).sort((a, b) => b[1].playCount - a[1].playCount);
    await writeTextFile(`${YEARLY_DATA_PATH(year)}/topArtists.json`, JSON.stringify(sortedArtists, null, 2), { baseDir: BaseDirectory.AppData });
    let sortedAlbums = Object.entries(yearlyAlbumAggregates).sort((a, b) => b[1].playCount - a[1].playCount);
    await writeTextFile(`${YEARLY_DATA_PATH(year)}/topAlbums.json`, JSON.stringify(sortedAlbums, null, 2), { baseDir: BaseDirectory.AppData });
    console.log(`Processed ${year}`);
  }

  console.log('Top Items Successfully Saved');
  await store.set('full-history-processed', true);
}
