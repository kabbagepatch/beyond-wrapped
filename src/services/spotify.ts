import axios from "axios";
import { getUserAccessToken } from "./connectToSpotify";

const clientId = import.meta.env.VITE_SPOTIFY_CLIENT_ID;
const clientSecret = import.meta.env.VITE_SPOTIFY_CLIENT_SECRET;

export const getBasicToken = async (): Promise<string> => {
  const url = `https://accounts.spotify.com/api/token?grant_type=client_credentials&client_id=${clientId}&client_secret=${clientSecret}`;
  console.log(url);
  const response = await axios.post(url, null, {
    headers: {
      "Content-Type": "application/x-www-form-urlencoded",
    },
  });
  console.log(response.data);
  return response.data.access_token;
}

const BASE_URL = 'https://api.spotify.com/v1';

const makeUserAuthorizedRequest = async (endpoint: string, method: string, data: any = null, params: any = null) => {
  const accessToken = await getUserAccessToken();
  if (!accessToken) {
    throw new Error("No access token available for user-authorized request.");
  }
  const url = `${BASE_URL}${endpoint}`;
  const response = await axios({
    method,
    url,
    headers: {
      Authorization: `Bearer ${accessToken}`,
    },
    data,
    params,
  });

  return response.data;
}

export const getPlaylist = async (playlistId: string) => {
  const accessToken = await getBasicToken();
  const url = `https://api.spotify.com/v1/playlists/${playlistId}`;
  const response = await axios.get(url, {
    headers: {
      Authorization: `Bearer ${accessToken}`,
    },
  });
  return response.data;
}

export const getUser = async () => {
  return await makeUserAuthorizedRequest('/me', 'GET');
}

export const getUserPlaybackState = async () => {
  return makeUserAuthorizedRequest('/me/player', 'GET');
}

export const getUserRecentlyPlayed = async () => {
  return makeUserAuthorizedRequest('/me/player/recently-played', 'GET', null, { limit: 1 });
}

export const getDevices = async () => {
  return makeUserAuthorizedRequest('/me/player/devices', 'GET');
}

export const getUserQueue = async () => {
  return makeUserAuthorizedRequest('/me/player/queue', 'GET');
}

export const getUserCurrentlyPlaying = async () => {
  return makeUserAuthorizedRequest('/me/player/currently-playing', 'GET');
}

export const skipToNextTrack = async () => {
  return makeUserAuthorizedRequest('/me/player/next', 'POST');
}

export const skipToPreviousTrack = async () => {
  return makeUserAuthorizedRequest('/me/player/previous', 'POST');
}

export const togglePlayPause = async (isPlaying: boolean, deviceId = '') => {
  return makeUserAuthorizedRequest(`/me/player/${isPlaying ? 'pause' : 'play'}`, 'PUT', { device_id: deviceId });
}

export const getTopItems = async (type: 'artists' | 'tracks', time_range: 'short_term' | 'medium_term' | 'long_term', limit: number = 20) => {
  return makeUserAuthorizedRequest(`/me/top/${type}?time_range=${time_range}&limit=${limit}`, 'GET');
}
