<template>
  <div class="playlist-container">
    <h1>{{ playlist.name.length > 24 ? playlist.name.substr(0, 24) + '...' : playlist.name }}</h1>
    <div class="tracks">
      <div :class="`track ${curIndex === i ? 'selected' : ''}`" v-for="(item, i) in playlist.tracks.items.filter((item: any) => item.track.id)" :key="item.track.id" @click="trackClick(item.track, i)">
        <img class="track-thumbnail" :src="item.track?.album?.images[2].url" alt="Album Art" />
        <div class="track-name" >{{ item.track.name }}</div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";

defineProps({
  playlist: {},
  curIndex: {
    type: Number,
    default: -1,
  },
  trackClick: {
    type: Function,
    default: () => {},
  },
});

</script>

<style scoped>
.playlist-container {
  width: var(--width);
  height: 400px;
  overflow-x: hidden;
  color: var(--text-color);
  margin-bottom: 8px;
}

h1 {
  margin: 0 5px;
  margin-bottom: 10px;
  font-size: 20px;
  width: 100%;
}

.tracks {
  height: 360px;
  overflow-y: auto;
  overflow-x: hidden;
  -ms-overflow-style: none;
  scrollbar-width: none;
}

.tracks::-webkit-scrollbar {
  display: none;
}

.track {
  display: flex;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px solid var(--secondary-color);
  width: 100%;
  padding-left: 5px;
  cursor: pointer;
}

.track-thumbnail {
  width: 48px;
  height: 48px;
  border-radius: 8px;
  margin-right: 12px;
}

.track-name {
  font-size: 18px;
}

.selected {
  color: var(--secondary-color);
  font-weight: bold;
  background-color: #31001438;
}
</style>