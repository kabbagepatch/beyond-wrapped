<template>
  <div class="container">
    <Header :title="track" icon="note-transparent" />
    <div v-if="plays" class="summary">
      <div>Track Name: <span class="summary-title">{{ plays[0].trackName }}</span></div>
      <div>Artist: <router-link :to="`/artists/${encodeURIComponent(plays[0].artistName)}`"><span class="summary-title">{{ plays[0].artistName }}</span></router-link></div>
      <div>Album: <span class="summary-title">{{ plays[0].albumName }}</span></div>
      <div>Number of times played: <span class="summary-title">{{ plays.length }}</span></div>
      <div>First Played on: <span class="summary-title">{{ new Date(plays[0].timeStamp).toLocaleDateString('en-US', dateOptions) }}</span></div>
    </div>
    <Stats :counts="playCounts" />
    <card v-if="plays" class="plays-card">
      <h1>All Plays</h1>
      <div class="plays" v-for="play in (showAll ? plays : plays.slice(0, 15))">
        <div>
          {{ new Date(play.timeStamp).toLocaleDateString('en-US', dateOptions) }}
          {{ new Date(play.timeStamp).toLocaleTimeString('en-US', timeOptions) }}
        </div>
        <div>
          {{ `${Math.floor(play.msPlayed / 60000)}m ${Math.floor((play.msPlayed % 60000) / 1000)}s` }}</div>
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
import { PlayEntry, useTrackerStore } from "../stores/tracker.ts";
import Stats from "../components/Stats.vue";

const trackerStore = useTrackerStore();
const route = useRoute();
const track = route.params.track as string;
const artist = route.params.artist as string;

const dateOptions = { year: 'numeric', month: '2-digit', day: '2-digit' } as const;
const timeOptions = { hour: 'numeric', minute: '2-digit' } as const;
const playCounts = ref<{ label: string, count: number }[]>([]);

const plays = ref<PlayEntry[]>();
trackerStore.getTrackPlays(track, artist).then(data => {
  plays.value = data;
}).catch(e => { console.log(e) });

trackerStore.getTrackPlayCounts(track, artist).then(data => {
  playCounts.value = data;
}).catch(e => { console.log(e) });

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