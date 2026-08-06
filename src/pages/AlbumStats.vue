<template>
  <div class="container">
    <Header :title="albumKey.split(' - ')[0]" icon="note-transparent" />
    <Stats
      v-if="album"
      :summary="[
        { key: 'Discovered on', value: album.firstTracks[0].firstPlayed },
        { key: 'Distinct tracks played', value: album.trackPlays.length },
        { key: 'Total number of plays', value: album.totalPlays },
        { key: 'Total Time played', value: album.timePlayed },
      ]"
      :cardOne="{ title: 'Times Played', entries: album.trackPlays.map(track => ({ left: track.trackName, right: track.playCount, link: track.key })), linkType: 'tracks' }"
      :cardTwo="{ title: 'First Played', entries: album.firstTracks.map(track => ({ left: track.trackName, right: track.firstPlayed, link: track.key })), linkType: 'tracks' }"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";
import Header from "../Header.vue";
import Stats from "../components/Stats.vue";
import { AlbumStats, useTrackerStore } from "../stores/tracker.ts";

const trackerStore = useTrackerStore();
const route = useRoute();
const albumKey = route.params.album as string;

const album = ref<AlbumStats['']>();
trackerStore.getAlbumStats(albumKey).then(data => {
  album.value = data;
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