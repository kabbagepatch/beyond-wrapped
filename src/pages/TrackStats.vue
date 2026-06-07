<template>
  <div class="container">
    <Header :title="trackKey.split(' - ')[0]" icon="note-transparent" />
    <div v-if="track" class="summary">
      <div>Track Name: <span class="summary-title">{{ track.info.trackName }}</span></div>
      <div>Artist: <router-link :to="`/artists/${encodeURIComponent(track.info.artistName)}`"><span class="summary-title">{{ track.info.artistName }}</span></router-link></div>
      <div>Album: <span class="summary-title">{{ track.info.albumName }}</span></div>
      <div>Number of times played: <span class="summary-title">{{ track.plays.length }}</span></div>
      <div>First Played on: <span class="summary-title">{{ track.firstPlayed }}</span></div>
    </div>
    <card v-if="track" class="plays-card">
      <h1>Plays</h1>
      <div class="plays" v-for="play in (showAll ? track.plays : track.plays.slice(0, 12))">
        <div>{{ play.dateString }}</div>
        <div>{{ play.timeString }} for {{ play.timePlayed }}</div>
      </div>
      <button class="plays" @click="showAll = !showAll">
        <div>
          {{ showAll ? 'Less' : 'More' }}...
        </div>
      </button>
    </card>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";
import Header from "../Header.vue";
import Card from "../components/Card.vue";
import { TrackStats, useTrackerStore } from "../stores/tracker.ts";

const trackerStore = useTrackerStore();
const route = useRoute();
const trackKey = route.params.track as string;

const track = ref<TrackStats['']>();
trackerStore.getTrackStats(trackKey).then(data => {
  track.value = data;
});

const showAll = ref(false);

</script>

<style scoped>
.summary {
  margin-bottom: 10px;
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
}

.plays {
  color: var(--background-color);
  margin-bottom: 5px;  
  display: flex;
  justify-content: space-between;
  font-family: monospace;
  font-size: 16px;
  text-shadow: none;
  font-weight: 600;
}
</style>