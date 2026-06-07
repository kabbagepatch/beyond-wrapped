<template>
  <div class="container">
    <Header title="Monthly Stats" icon="month" />
    <TimeStats
      :month="monthString"
      :year="year"
      :topSongs="topTracks"
      :topArtists="topArtists"
      :topAlbums="topAlbums"
      :totalSongs="totalSongs"
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

const trackerStore = useTrackerStore();
const router = useRouter();
const route = useRoute();
const month = route.params.month as string;
const monthString = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"][parseInt(month) - 1];
const year = route.params.year as string;

const topTracks = ref<TotalsList>([]);
const topArtists = ref<TotalsList>([]);
const topAlbums = ref<TotalsList>([]);
const totalSongs = ref(0);
const totalTime = ref(0);
trackerStore.getTopTracks(year, month).then(data => {
  topTracks.value = data;
  topTracks.value.forEach(song => {
    totalSongs.value += song[1].playCount;
    totalTime.value += song[1].msPlayed;
  });
});
trackerStore.getTopArtists(year, month).then(data => {
  topArtists.value = data;
});
trackerStore.getTopAlbums(year, month).then(data => {
  topAlbums.value = data;
});

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