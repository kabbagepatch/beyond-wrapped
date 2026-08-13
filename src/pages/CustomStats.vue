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
      :trackPlays="trackPlays"
      :artistPlays="artistPlays"
      :albumPlays="albumPlays"
      :counts="playCounts"
      :totalPlayCount="totalPlayCount"
      :totalTime="totalTime"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRoute } from "vue-router";
import Header from "../Header.vue";
import TimeStats from "../components/TimeStats.vue";
import { ItemPlayData, PlayCounts, useTrackerStore } from "../stores/tracker.ts";

const trackerStore = useTrackerStore();
const route = useRoute();
const from = route.query.from as string || '9-2012';
const to = route.query.to as string || '12-2025';

const trackPlays = ref<ItemPlayData[]>([]);
const artistPlays = ref<ItemPlayData[]>([]);
const albumPlays = ref<ItemPlayData[]>([]);
const playCounts = ref<PlayCounts>([]);
const totalPlayCount = ref(0);
const totalTime = ref(0);

trackerStore.getTopItemsCustom('tracks', from, to).then(data => {
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

trackerStore.getTopItemsCustom('artists', from, to).then(data => {
  artistPlays.value = data;
}).catch(e => { console.log(e) });

trackerStore.getTopItemsCustom('albums', from, to).then(data => {
  albumPlays.value = data;
}).catch(e => { console.log(e) });

trackerStore.getCustomCounts(from, to).then(data => {
  playCounts.value = data;
}).catch(e => { console.log(e) });

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