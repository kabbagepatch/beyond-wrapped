<template>
  <div class="container">
    <Header :title="artist" icon="note-transparent" />
    <div class="summary">
      Discovered on:
      <span class="summary-title">{{ dateSortedStats.length ? new Date(dateSortedStats[0].firstPlay).toLocaleDateString('en-US', dateOptions) : '' }}</span>
    </div>
    <Stats
      v-if="artist"
      :summary="[
        { key: 'Distinct tracks played', value: stats.length },
        { key: 'Total number of plays', value: totalPlays },
        { key: 'Total Time played', value: timePlayed },
      ]"
      :counts="playCounts"
      :cardOne="{ title: 'Top Tracks', entries: stats.map(track => ({
        left: track.trackName,
        right: track.playCount,
        link: `/artists/${encodeURIComponent(track.artistName)}/tracks/${encodeURIComponent(track.trackName)}`
      })) }"
      :cardTwo="{ title: 'First Tracks', entries: dateSortedStats.map(track => ({
        left: track.trackName,
        right: new Date(track.firstPlay).toLocaleDateString('en-US', dateOptions),
        link: `/artists/${encodeURIComponent(track.artistName)}/tracks/${encodeURIComponent(track.trackName)}`
      })) }"
      :cardThree="{ title: 'All Plays', entries: plays.map(play => ({
        left: play.trackName,
        right: new Date(play.timeStamp).toLocaleDateString('en-US', dateOptions),
        link: `/artists/${encodeURIComponent(play.artistName)}/tracks/${encodeURIComponent(play.trackName)}`
      })) }"
      :smallLeftEntry="true"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";
import Header from "../Header.vue";
import Stats from "../components/Stats.vue";
import { PlayEntry, TrackStats, useTrackerStore } from "../stores/tracker.ts";

const trackerStore = useTrackerStore();
const route = useRoute();
const artist = route.params.artist as string;

const dateOptions = { year: 'numeric', month: '2-digit', day: '2-digit' } as const;

const stats = ref<TrackStats[]>([]);
const dateSortedStats = ref<TrackStats[]>([]);
const totalPlays = ref<number>(0);
const timePlayed = ref<string>('');
const playCounts = ref<{ label: string, count: number }[]>([]);

trackerStore.getArtistStats(artist).then(data => {
  stats.value = data;

  let totalMsPlayed = 0;
  let totalPlaysLocal = 0;
  data.forEach(stat => {
    totalPlaysLocal += stat.playCount;
    totalMsPlayed += stat.msPlayed;
  });
  dateSortedStats.value = ([] as TrackStats[]).concat(data);
  dateSortedStats.value.sort((a, b) => a.firstPlay.localeCompare(b.firstPlay));

  totalPlays.value = totalPlaysLocal;
  const totalHours = Math.floor(totalMsPlayed / 3600000);
  const totalMinutes = Math.floor((totalMsPlayed % 3600000) / 60000);
  const totalSeconds = Math.floor((totalMsPlayed % 60000) / 1000);
  timePlayed.value = `${totalHours}h ${totalMinutes}m ${totalSeconds}s`;
}).catch(e => { console.log(e) });

const plays = ref<PlayEntry[]>([]);
trackerStore.getArtistPlays(artist).then(data => {
  plays.value = data;
}).catch(e => { console.log(e) });

trackerStore.getArtistPlayCounts(artist).then(data => {
  playCounts.value = data;
}).catch(e => { console.log(e) });

</script>

<style scoped>
.summary {
  margin-left: 10px;
  font-family: monospace;
  font-weight: bold;
}

.summary-title {
  color: var(--primary-color);
}

.plays-card {
  text-align: left;
  font-weight: bold;
  margin: 10px 0;
}

.plays {
  color: var(--background-color);
  margin-bottom: 2px;
  display: flex;
  justify-content: space-between;
}

.title-button {
  display: block;
  height: 30px;
  width: 105%;
  margin-top: -5px;
  margin-left: -10px;
  margin-bottom: 5px;
  border-radius: 15px;
}

.title-button:hover {
  background-color: hsla(227, 8%, 22%, 25%);
}

.title {
  text-align: left;
  margin-left: 10px;
}
</style>