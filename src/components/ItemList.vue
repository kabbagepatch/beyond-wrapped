<template>
  <div class="items-container">
    <div class="items">
      <div
        class="item"
        v-for="(item, i) in items.filter((item: any) => item.id)"
        :key="item.id"
        @click="itemClick(item, i)"
      >
        <img
          v-if="item.images || item.album?.images"
          class="item-thumbnail"
          :src="item.images ? item.images[2].url : item.album?.images[2].url"
          alt="Album Art"
        />
        <div class="item-name" >{{ item.name.length > 30 ? item.name.slice(0, 30) + '...' : item.name }}</div>
      </div>
      <div class="item" @click="onMore">
        <div class="more" >{{ items.length < 50 ? 'More...' : 'Less' }}</div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";

defineProps({
  title: {
    type: String,
    default: "",
  },
  items: {
    type: Array,
    default: () => ([]),
  },
  itemClick: {
    type: Function,
    default: () => {},
  },
  onMore: {
    type: Function,
    default: () => {},
  }
});

</script>

<style scoped>
.items-container {
  width: var(--width);
  overflow-x: hidden;
  color: var(--text-color);
  margin-bottom: 8px;
  font-size: 16px;
  font-family: monospace;
  text-shadow: none;
  color: var(--background-color);
}

.items {
  overflow-y: auto;
  overflow-x: hidden;
  -ms-overflow-style: none;
  scrollbar-width: none;
  border-radius: 12px;
}

.items::-webkit-scrollbar {
  display: none;
}

.item {
  display: flex;
  align-items: center;
  padding: 4px 0;
  background-color: var(--primary-color);
  border-bottom: 1px solid var(--secondary-color);
  width: 100%;
  cursor: pointer;
}

.item-thumbnail {
  width: 35px;
  height: 35px;
  border-radius: 8px;
  margin-left: 12px;
}

.item-index {
  width: 10px;
  font-size: 12px;
}

.item-name, .item-index {
  padding: 2px 0;
  margin-left: 12px;
  font-weight: bold;
}

.selected {
  color: var(--secondary-color);
  font-weight: bold;
  background-color: #31001438;
}

.more {
  width: 100%;
  text-align: center;
}
</style>
