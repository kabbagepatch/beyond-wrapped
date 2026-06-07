<template>
  <div class="container">
    <div v-if="displaySummary" class="summary">
      <div v-for="item in summary">
        <div><span class="summary-value">{{ item.value }}</span> {{ item.key }}</div>
      </div>
    </div>
    <stats-card
      :card="cardOne"
      :visible="displayCardOne"
      :entries="displaySummary ? 5 : 200"
      @toggle="toggleCardOne"
    />
    <stats-card
      :card="cardTwo"
      :visible="displayCardTwo"
      :entries="displaySummary ? 5 : 200"
      @toggle="toggleCardTwo"
    />
    <stats-card
      :card="cardThree"
      :visible="displayCardThree"
      :entries="displaySummary ? 5 : 200"
      @toggle="toggleCardThree"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import StatsCard from "./StatsCard.vue";

defineProps<{
  summary: { key: string; value: string | number | undefined }[],
  cardOne: { title: string; entries: { left: string, right: string | number | undefined, link?: string }[], linkType?: 'tracks' | 'artists' | 'albums' },
  cardTwo?: { title: string; entries: { left: string, right: string | number | undefined, link?: string }[], linkType?: 'tracks' | 'artists' | 'albums' },
  cardThree?: { title: string; entries: { left: string, right: string | number | undefined, link?: string }[], linkType?: 'tracks' | 'artists' | 'albums' },
}>();

const displaySummary = ref(true);
const displayCardOne = ref(true);
const displayCardTwo = ref(true);
const displayCardThree = ref(true);

const toggleCardOne = () => {
  displaySummary.value = !displaySummary.value;
  displayCardTwo.value = !displayCardTwo.value;
  displayCardThree.value = !displayCardThree.value;
}

const toggleCardTwo = () => {
  displaySummary.value = !displaySummary.value;
  displayCardOne.value = !displayCardOne.value;
  displayCardThree.value = !displayCardThree.value;
}

const toggleCardThree = () => {
  displaySummary.value = !displaySummary.value;
  displayCardOne.value = !displayCardOne.value;
  displayCardTwo.value = !displayCardTwo.value;
}

</script>

<style scoped>
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 10px;
}

.header h1 {
  text-align: center;
  margin: 0;
}

.summary {
  margin-bottom: 10px;
  margin-left: 10px;
  font-family: monospace;
  font-weight: bold;
}

.summary-value {
  color: var(--primary-color);
}
</style>