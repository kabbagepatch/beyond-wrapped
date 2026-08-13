<template>
  <div class="container">
    <div v-if="displaySummary" class="summary">
      <div v-for="item in summary">
        <div><span class="summary-value">{{ item.value }}</span> {{ item.key }}</div>
      </div>
    </div>
    <div v-if="displaySummary && counts?.length" class="bar-section">
      <h1 class="title">{{ counts[0].label.length === 4 ? 'Yearly' : (counts[0].label.length === 1 ? 'Monthly' : 'Daily')}} Plays</h1>
      <div class="bar-chart">
        <div
          class="bar-col"
          v-for="count in counts"
          :key="count.label"
        >
          <span class="bar-count" v-if="count.count">{{ count.count }}</span>
          <div
            class="bar"
            :style="{ height: `${barHeight(count.count, maxCount)}px` }"
          />
          <span class="bar-label">{{ count.label }}</span>
        </div>
      </div>
    </div>
    <stats-card
      :card="cardOne"
      :visible="displayCardOne"
      :entryCount="displaySummary ? (entryCount ?? 5) : 1000"
      @toggle="toggleCardOne"
    />
    <stats-card
      :card="cardTwo"
      :visible="displayCardTwo"
      :entryCount="displaySummary ? (entryCount ?? 5) : 1000"
      @toggle="toggleCardTwo"
      :smallLeftEntry="smallLeftEntry"
    />
    <stats-card
      :card="cardThree"
      :visible="displayCardThree"
      :entryCount="displaySummary ? (entryCount ?? 5) : 1000"
      @toggle="toggleCardThree"
      :smallLeftEntry="smallLeftEntry"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import StatsCard from "./StatsCard.vue";
import { PlayCounts } from "../stores/tracker.ts";

const props = defineProps<{
  summary?: { key: string; value: string | number | undefined }[],
  counts?: PlayCounts,
  cardOne?: { title: string; entries: { left: string, right: string | number | undefined, link?: string }[] },
  cardTwo?: { title: string; entries: { left: string, right: string | number | undefined, link?: string }[] },
  cardThree?: { title: string; entries: { left: string, right: string | number | undefined, link?: string }[] },
  smallLeftEntry?: boolean,
  entryCount?: number,
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

const maxCount = computed(() => props.counts ? Math.max(...props.counts.map(m => m.count), 1) : 0);
const barHeight = (count: number, max: number) => count === 0 ? 2 : Math.max(4, Math.round((count / max) * 80));

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

.bar-section {
  margin: 6px 10px;
}

.bar-section h1 {
  color: var(--text-color);
  margin-bottom: 20px;
}

.bar-chart {
  display: flex;
  align-items: flex-end;
  gap: 4px;
  height: calc(80px + 30px);
}

.bar-col {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: flex-end;
  min-width: 0;
}

.bar {
  width: 100%;
  background-color: var(--primary-color);
  border-radius: 2px 2px 0 0;
  min-height: 2px;
}

.bar-count {
  font-size: 9px;
  color: var(--text-color);
  line-height: 1;
  margin-bottom: 4px;
}

.bar-label {
  font-size: 9px;
  color: var(--text-color);
  text-align: center;
  white-space: nowrap;
  overflow: hidden;
  width: 100%;
  text-overflow: ellipsis;
}
</style>