<template>
  <div class="container">
    <Header title="Yearly Stats" icon="year" />
    <TimeStats
      :year="year"
      :trackPlays="trackPlays"
      :artistPlays="artistPlays"
      :albumPlays="albumPlays"
      :totalPlayCount="totalPlayCount"
      :totalTime="totalTime"
      :counts="monthlyCounts"
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
import { ItemPlayData, PlayCounts, useTrackerStore } from "../stores/tracker.ts";

const router = useRouter();
const route = useRoute();
const year = route.params.year as string;

const trackerStore = useTrackerStore();

const blank: ItemPlayData = { primary: '', secondary: '', playCount: 0, msPlayed: 0 }
const trackPlays = ref<ItemPlayData[]>([blank, blank, blank, blank, blank]);
const artistPlays = ref<ItemPlayData[]>([blank, blank, blank, blank, blank]);
const albumPlays = ref<ItemPlayData[]>([blank, blank, blank, blank, blank]);
const monthlyCounts = ref<PlayCounts>([{ label: 'J', count: 0 }]);
const totalPlayCount = ref(0);
const totalTime = ref(0);

trackerStore.getTopTracks(parseInt(year, 10)).then(data => {
  trackPlays.value = data;
  let playCount = 0;
  let time = 0;

  for (const song of data) {
    playCount += song.playCount;
    time += song.msPlayed;
  }

  totalPlayCount.value = playCount;
  totalTime.value = time;
}).catch(e => { console.log(e) });

trackerStore.getTopArtists(parseInt(year, 10)).then(data => {
  artistPlays.value = data;
}).catch(e => { console.log(e) });

trackerStore.getTopAlbums(parseInt(year, 10)).then(data => {
  albumPlays.value = data;
}).catch(e => { console.log(e) });

trackerStore.getMonthlyCounts(parseInt(year, 10)).then(data => {
  monthlyCounts.value = data;
}).catch(e => { console.log(e) });

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