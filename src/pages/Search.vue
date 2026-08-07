<template>
  <div class="container">
    <Header title="Search" icon="note-transparent" />
    <form class="search-bar" @submit="search">
      <input v-model.trim="searchString" placeholder="Search for tracks, artists or albums..." />
      <button class="search-button">Search</button>
    </form>
    <stats
      :summary="status === 'complete' && combined.length === 0 ? [{ key: 'Results Found', value: 'No' }] : undefined"
      :cardTwo="{ title: 'Results', entries: combined.map(entry => ({
        left: entry.display,
        right: entry.type,
        link: entry.link,
      })) }"
      :smallLeftEntry="true"
      :entryCount="20"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';

import Header from "../Header.vue";
import Stats from '../components/Stats.vue';
import { useTrackerStore } from '../stores/tracker';

const router = useRouter();
const route = useRoute();

let searchString = ref(route.query.value as string);
let combined = ref<any[]>([]);
let status = ref('init');

const trackerStore = useTrackerStore();

const search = async (e: SubmitEvent) => {
  e.preventDefault();
  router.replace({ query: { value: searchString.value } });
}

const callSearch = async () => {
  status.value = 'in-progress'
  try {
    const results = await trackerStore.searchItems(searchString.value);
    combined.value = [
      ...results.tracks.map((t: any)  => ({
        ...t,
        type: 'Track',
        display: `${t.name} - ${t.artist}`,
        link: `artists/${encodeURIComponent(t.artist)}/tracks/${encodeURIComponent(t.name)}`
      })),
      ...results.artists.map((a: any) => ({
        ...a,
        type: 'Artist',
        display: `${a.name}`,
        link: `artists/${encodeURIComponent(a.name)}`
      })),
      ...results.albums.map((a: any)  => ({
        ...a,
        type: 'Album',
        display: `${a.name} - ${a.artist}`,
        link: `artists/${encodeURIComponent(a.artist)}/albums/${encodeURIComponent(a.name)}`
      })),
    ].sort((a, b) => b.play_count - a.play_count);
    console.log(combined.value.length);
    status.value = 'complete'
  } catch(e) {
    console.log('failure');
    console.log(e);
    status.value = 'error'
  }
}

if (searchString.value) callSearch();

</script>

<style scoped>
.search-bar {
  display: flex;
  min-width: var(--width);
  width: 100%;
  margin-bottom: 10px;
}

.search-bar input {
  width: 100%;
  display: block;
}

.search-button {
  background-color: var(--primary-color);
  border: 1px solid var(--secondary-color);
  color: var(--secondary-color);
  padding: 5px;
}
</style>
