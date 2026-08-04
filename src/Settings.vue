<template>
  <main class="container">
    <div class="home">
      <Header />
      <div class="description">
        <p>
          Spotify Wrapped feels so limited. Get a better insight into your listening history here.
          Request your Extended Listening History from Spotify <a href="https://www.spotify.com/us/account/privacy/" target="_blank">here</a>.
          Then upload the zip file you get from Spotify below. Wait for the processing and check out all your stats <a href="/">here</a>.
        </p>
      </div>
      
      <button @click="uploadZip">
        <title-card
          title="Upload Spotify History"
          iconName="cells"
          :subtitles="[
            'Upload a zip file containing the Spotify Extended Streaming History folder',
            `Last Uploaded: ${lastUpload}`,
          ]"
        />
      </button>
      <div class="status alert" v-if="status === 'in-progress'">Upload in Progress. Do not close application</div>
      <div class="status" v-if="status === 'uploaded'">Upload complete. Processing...</div>
      <div class="status" v-if="status === 'complete'">Processing complete.</div>
      <div class="status alert" v-if="status === 'error'">Upload failed</div>
    </div>
      
    <button @click="toggleTheme">
      <div class="theme-button">
        <img class="icon" src="./assets/icons/paint-brush.png" />
        <div class="theme-button-text">Toggle Theme</div>
      </div>
    </button>
  </main>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import * as tauristore from '@tauri-apps/plugin-store';

import TitleCard from "./components/TitleCard.vue";
import Header from "./Header.vue";

import { setTheme } from "./themes.ts";
import { ref } from "vue";

let curTheme = localStorage.getItem('theme') || 'Forest';
const toggleTheme = () => {
  if (curTheme === 'Coffee') {
    curTheme = 'Fairy';
  } else if (curTheme === 'Forest') {
    curTheme = 'Coffee';
  } else {
    curTheme = 'Forest';
  }
  setTheme(curTheme);
  localStorage.setItem('theme', curTheme);
}

const lastUpload = ref('Never');
const status = ref('init');
const getLastUpload = async () => {
  const store = await tauristore.load('store.json');
  const lastUploadValue = await store.get<number>('last-upload-history');
  if (lastUploadValue) {
    lastUpload.value = `${new Date(lastUploadValue).toDateString()}`;
  }
}
getLastUpload();

const uploadZip = async (_: Event) => {
  const selected = await open({
    multiple: false,
    filters: [{ name: 'ZIP Archive', extensions: ['zip'] }]
  });

  if (selected) {
    lastUpload.value = 'Upload in progress...';
    status.value = 'in-progress';
    try {
      await invoke('process_zip_file', { filePath: selected });
      await getLastUpload();
      status.value = 'uploaded';
      await invoke('process_raw_history');
      status.value = 'complete';
    } catch (e) {
      console.error(e);
      status.value = 'error';
      lastUpload.value = 'There was an error during the upload';
    }
  }
}

</script>

<style scoped>
.home {
  width: 100%;
}

.description {
  padding: 0 4px;
  margin-top: 10px;
}

a {
  color: hsl(227, 96%, 69%);
}

button, label {
  display: block;
  margin-top: 20px;
}

.theme-button {
  max-width: var(--width);
  display: flex;
  align-items: center;
  border-radius: 12px;
  background-color: var(--primary-color);
  text-shadow: -1.5px -1.5px 0 var(--text-outline), 1.5px -1.5px 0 var(--text-outline), -1.5px 1.5px 0 var(--text-outline), 1.5px 1.5px 0 var(--text-outline);
  padding: 5px 10px;
}

.icon {
  margin: 5px 10px;
  height: 30px;
}

.file-upload {
  cursor: pointer;
}

.status {
  width: 100%;
  text-align: center;
  margin-top: 15px;
  font-weight: bold;
  color: var(--primary-color);
}

.alert {
  color: rgb(255, 84, 84);
}
</style>