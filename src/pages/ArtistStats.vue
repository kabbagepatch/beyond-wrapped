<template>
  <div class="container">
    <Header :title="artistKey" icon="note-transparent" />
    <Stats
      v-if="artist"
      :summary="[
        { key: 'Discovered on', value: artist.firstTracks[0].firstPlayed },
        { key: 'Distinct tracks played', value: artist.trackPlays.length },
        { key: 'Total number of plays', value: artist.totalPlays },
        { key: 'Total Time played', value: artist.timePlayed },
      ]"
      :cardOne="{ title: 'Top Tracks', entries: artist.trackPlays.map(track => ({ left: track.trackName, right: track.playCount, link: track.key })), linkType: 'tracks' }"
      :cardTwo="{ title: 'First Tracks', entries: artist.firstTracks.map(track => ({ left: track.trackName, right: track.firstPlayed, link: track.key })), linkType: 'tracks' }"
      :smallLeftEntry="true"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";
import Header from "../Header.vue";
import Stats from "../components/Stats.vue";
import { ArtistStats, useTrackerStore } from "../stores/tracker.ts";

const trackerStore = useTrackerStore();
const route = useRoute();
const artistKey = route.params.artist as string;

const artist = ref<ArtistStats['']>();
trackerStore.getArtistStats(artistKey).then(data => {
  artist.value = data;
});

</script>

<style scoped>
.summary {
  font-size: 18px;
  line-height: 28px;
  margin-left: 10px;
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