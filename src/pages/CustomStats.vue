<template>
  <div class="container">
    <Header title="Custom Range" icon="calendar" />
    <TimeStats
      :custom="{
        fromMonth: from.split('-')[0],
        fromYear: from.split('-')[1],
        toMonth: to.split('-')[0],
        toYear: to.split('-')[1],
      }"
      :topSongs="topTracks"
      :topArtists="topArtists"
      :topAlbums="topAlbums"
      :totalSongs="totalSongs"
      :totalTime="totalTime"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";
import Header from "../Header.vue";
import TimeStats from "../components/TimeStats.vue";
import { TotalsList, useTrackerStore } from "../stores/tracker.ts";

const trackerStore = useTrackerStore();
const route = useRoute();
const from = route.query.from as string || '9-2012';
const to = route.query.to as string || '12-2025';

const topTracks = ref<TotalsList>([]);
const topArtists = ref<TotalsList>([]);
const topAlbums = ref<TotalsList>([]);
const totalSongs = ref(0);
const totalTime = ref(0);
trackerStore.getTopItemsCustom('tracks', from, to).then(data => {
  topTracks.value = data;
  topTracks.value.forEach(song => {
    totalSongs.value += song[1].playCount;
    totalTime.value += song[1].msPlayed;
  });
});
trackerStore.getTopItemsCustom('artists', from, to).then(data => {
  topArtists.value = data;
});
trackerStore.getTopItemsCustom('albums', from, to).then(data => {
  topAlbums.value = data;
});

</script>

<style scoped>
.header h1 {
  text-align: center;
  margin: 0;
}

h1 {
  margin-bottom: 5px;
}

.summary {
  margin-bottom: 10px;
}

.summary-title {
  color: var(--primary-color);
}

.title {
  text-align: left;
  margin-left: 0;
}

.entry {
  text-align: left;
  text-shadow: none;
  color: var(--background-color);
  margin: 0;
  font-weight: bold;
}
</style>