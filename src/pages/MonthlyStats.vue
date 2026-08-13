<template>
  <div class="container">
    <Header title="Monthly Stats" icon="month" />
    <TimeStats
      :month="monthString"
      :year="year"
      :trackPlays="trackPlays"
      :artistPlays="artistPlays"
      :albumPlays="albumPlays"
      :totalPlayCount="totalPlayCount"
      :totalTime="totalTime"
      :counts="dailyCounts"
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

const trackerStore = useTrackerStore();
const router = useRouter();
const route = useRoute();
const month = route.params.month as string;
const monthString = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"][parseInt(month) - 1];
const year = route.params.year as string;

const blank: ItemPlayData = { primary: '', secondary: '', playCount: 0, msPlayed: 0 }
const blankC: PlayCounts = [{ label: '1', count: 0 }]
const trackPlays = ref<ItemPlayData[]>([blank, blank, blank, blank, blank]);
const artistPlays = ref<ItemPlayData[]>([blank, blank, blank, blank, blank]);
const albumPlays = ref<ItemPlayData[]>([blank, blank, blank, blank, blank]);
const dailyCounts = ref<PlayCounts>([{ label: 'J', count: 0 }]);
const totalPlayCount = ref(0);
const totalTime = ref(0);

trackerStore.getTopTracks(parseInt(year, 10), parseInt(month, 10)).then(data => {
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

trackerStore.getTopArtists(parseInt(year, 10), parseInt(month, 10)).then(data => {
  artistPlays.value = data;
}).catch(e => { console.log(e) });

trackerStore.getTopAlbums(parseInt(year, 10), parseInt(month, 10)).then(data => {
  albumPlays.value = data;
}).catch(e => { console.log(e) });

trackerStore.getDailyCounts(parseInt(year, 10), parseInt(month, 10)).then(data => {
  dailyCounts.value = data;
}).catch(e => { console.log(e) });

const forward = () => {
  let newYear = parseInt(year, 10);
  let nextMonth = parseInt(month, 10) + 1;
  if (nextMonth > 12) {
    newYear += 1
    nextMonth = 1;
  };
  router.replace(`/year/${newYear}/${nextMonth}`)
}

const back = () => {
  let newYear = parseInt(year, 10);
  let prevMonth = parseInt(month, 10) - 1;
  if (prevMonth < 1) {
    newYear -= 1
    prevMonth = 12;
  };
  router.replace(`/year/${newYear}/${prevMonth}`)
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