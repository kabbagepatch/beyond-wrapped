<template>
  <main class="container">
    <div class="home">
      <Header />
      <button @click="toggleTheme">
        <div class="theme-button">
          <img class="icon" src="./assets/icons/paint-brush.png" />
          <div class="theme-button-text">Toggle Theme</div>
        </div>
      </button>
      
      <button @click="uploadZip">
        <title-card
          title="Upload Spotify History"
          iconName="cells"
          :subtitles="[
            'Upload a zip file containing the Spotify Extended Streaming History folder',
            `Last Uploaded: ${lastUpload}`
          ]"
        />
      </button>
    </div>
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
    try {
      const response = await invoke('process_zip_file', { filePath: selected });
      console.log(response);
    } catch (e) {
      lastUpload.value = 'There was an error during the upload';
    }
    getLastUpload();
    invoke('process_raw_history');
  }
}

</script>

<style scoped>
.home {
  width: 100%;
}

button, label {
  display: block;
  margin-top: 20px;
}

.theme-button {
  width: var(--width);
  display: flex;
  align-items: center;
  border-radius: 12px;
  background-color: var(--primary-color);
}

.icon {
  margin: 5px 10px;
  height: 30px;
}

.file-upload {
  cursor: pointer;
}
</style>