<template>
  <div class="container">
    <Header title="Yearly Stats" icon="year" />
    <TimeStats
      :year="year"
      :topSongs="topTracks"
      :totalSongs="totalSongs"
      :topArtists="topArtists"
      :topAlbums="topAlbums"
      :totalTime="totalTime"
      :back="back"
      :forward="forward"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import Header from "../Header.vue";
import TimeStats from "../components/TimeStats.vue";
import { TotalsList, useTrackerStore } from "../stores/tracker.ts";

const router = useRouter();
const route = useRoute();
const year = route.params.year as string;

const trackerStore = useTrackerStore();

const topTracks = ref<TotalsList>([]);
const topArtists = ref<TotalsList>([]);
const topAlbums = ref<TotalsList>([]);
const totalSongs = ref(0);
const totalTime = ref(0);
trackerStore.getTopTracks(year).then(data => {
  topTracks.value = data;
  topTracks.value.forEach(song => {
    totalSongs.value += song[1].playCount;
    totalTime.value += song[1].msPlayed;
  });
});
trackerStore.getTopArtists(year).then(data => {
  topArtists.value = data;
});
trackerStore.getTopAlbums(year).then(data => {
  topAlbums.value = data;
});

const forward = () => {
  const nextYear = parseInt(year, 10) + 1;
  if (nextYear > 2026) return;
  router.replace(`/year/${nextYear}`)
}

const back = () => {
  const prevYear = parseInt(year, 10) - 1;
  if (prevYear < 2012) return;
  router.replace(`/year/${prevYear}`)
}

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