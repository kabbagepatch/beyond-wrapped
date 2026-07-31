<template>
  <div class="section" v-if="card?.entries?.length && visible">
    <card>
      <button class="title-button" @click="emit('toggle')"><h1 class="title">{{ card.title }}</h1></button>
      <div v-for="(entry, i) in card.entries.slice(0, entries)">
        <div :class="['entry', { 'first-entry': i === 0 }]">
          <router-link
            v-if="entry.link"
            :to="`/${card.linkType}/${encodeURIComponent(entry.link)}`"
          >
            <span :class="['left-entry', { 'left-entry-small': smallLeftEntry }]">{{ entry.left }}</span>
          </router-link>
          <div v-else ><span class="left-entry">{{ entry.left }}</span></div>
          <div class="right-entry">{{ entry.right }}</div>
        </div>
      </div>
    </card>
  </div>
</template>

<script setup lang="ts">
import Card from "../components/Card.vue";

defineProps<{
  card?: {
    title: string;
    entries: { left: string, right: string | number | undefined, link?: string }[],
    linkType?: 'tracks' | 'artists' | 'albums',
  }
  visible: boolean
  entries: number
  smallLeftEntry?: boolean
}>();

const emit = defineEmits<{
  toggle: []
}>()

</script>

<style scoped>
.section {
  margin-bottom: 10px;
  font-family: Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
}

.section:nth-last-child(1) {
  margin-bottom: 0;
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

.entry {
  display: flex;
  text-align: left;
  text-shadow: none;
  color: var(--background-color);
  margin: 0;
  font-weight: bold;
  font-family: monospace;
}

.entry a {
  color: var(--background-color);
}

.entry a:hover {
  background-color: hsla(227, 8%, 22%, 5%);
  color: var(--background-color-dark);
}

.first-entry {
  color: var(--tertiary-color);
}

.first-entry a {
  color: var(--tertiary-color);
}

.left-entry {
  width: calc(var(--width) - 90px);
  display: block;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.left-entry-small {
  width: calc(var(--width) - 140px);
}

.right-entry {
  width: 100%;
  text-align: right;
}
</style>