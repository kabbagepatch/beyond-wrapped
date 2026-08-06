<template>
  <div class="container">
    <div class="header custom-header" v-if="custom">
      <div>
        <dropdown :title="months[parseInt(custom.fromMonth, 10) - 1]" :options="months" :selectOption="selectFromMonth" />&nbsp;
        <dropdown :title="custom.fromYear" :options="years" :selectOption="selectFromYear" />
      </div>
      <div class="hyphen">-</div>
      <div>
        <dropdown :title="months[parseInt(custom.toMonth, 10) - 1]" :options="months" :selectOption="selectToMonth" />&nbsp;
        <dropdown :title="custom.toYear" :options="years" :selectOption="selectToYear" />
      </div>
    </div>
    <div class="header" v-else>
      <button @click="back"><</button>
      <div>
        <dropdown v-if="month" :title="month" :options="months" :selectOption="selectMonth" />&nbsp;
        <dropdown v-if="year" :title="year" :options="years" :selectOption="selectYear" />
      </div>
      <button @click="forward">></button>
    </div>
    <Stats
      :summary="[
        { key: 'Songs Played', value: totalPlayCount },
        { key: 'Spent Listening', value: `${Math.floor(totalTime / 3600000)}h ${Math.floor((totalTime % 3600000) / 60000)}m ${Math.floor((totalTime % 60000) / 1000)}s` },
        { key: 'Distinct Songs', value: trackPlays.length },
        { key: 'Distinct Artists', value: artistPlays.length },
        { key: 'Distinct Albums', value: albumPlays.length },
      ]"
      :cardOne="{
        title: 'Top Tracks',
        entries: trackPlays.map(track => ({
          left: `${track.primary} - ${track.secondary}`,
          right: track.playCount,
          link: `/artists/${encodeURIComponent(track.secondary ?? '')}/tracks/${encodeURIComponent(track.primary)}`
        })),
      }"
      :cardTwo="{
        title: 'Top Artists',
        entries: artistPlays.map(artist => ({
          left: artist.primary,
          right: artist.playCount,
          link: `/artists/${encodeURIComponent(artist.primary)}`
        })),
      }"
      :cardThree="{
        title: 'Top Albums',
        entries: albumPlays.map(album => ({
          left: `${album.primary} - ${album.secondary}`,
          right: album.playCount,
          link: `/artists/${encodeURIComponent(album.secondary ?? '')}/albums/${encodeURIComponent(album.primary)}`
        })),
      }"
    />
  </div>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";
import Stats from "./Stats.vue";
import Dropdown from "./Dropdown.vue";
import { PlayCount } from "../stores/tracker";

const router = useRouter();

const props = defineProps<{
  custom?: {
    fromMonth: string,
    fromYear: string,
    toMonth: string,
    toYear: string,
  },
  month?: string,
  year?: string,
  totalPlayCount: number,
  totalTime: number,
  trackPlays: PlayCount[],
  artistPlays: PlayCount[],
  albumPlays: PlayCount[],
  back?: () => void,
  forward?: () => void,
}>();

const years = Array.from({ length: 2026 - 2012 + 1 }, (_, i) => (2012 + i).toString()).reverse();
const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];

const selectYear = (year: string) => {
  if (props.month)
    router.replace(`/year/${year}/${months.indexOf(props.month) + 1}`);
  else
    router.replace(`/year/${year}`);
}
const selectMonth = (month: string) => {
  router.replace(`/year/${props.year}/${months.indexOf(month) + 1}`);
}

const selectFromYear = (year: string) => {
  router.replace(`/custom?from=${props.custom?.fromMonth}-${year}&to=${props.custom?.toMonth}-${props.custom?.toYear}`);
}
const selectFromMonth = (month: string) => {
  const monthIndex = (months.indexOf(month) + 1).toString();
  router.replace(`/custom?from=${monthIndex}-${props.custom?.fromYear}&to=${props.custom?.toMonth}-${props.custom?.toYear}`);
}
const selectToYear = (year: string) => {
  router.replace(`/custom?from=${props.custom?.fromMonth}-${props.custom?.fromYear}&to=${props.custom?.toMonth}-${year}`);
}
const selectToMonth = (month: string) => {
  const monthIndex = (months.indexOf(month) + 1).toString();
  router.replace(`/custom?from=${props.custom?.fromMonth}-${props.custom?.fromYear}&to=${monthIndex}-${props.custom?.toYear}`);
}

</script>

<style scoped>
.header {
  margin-bottom: 5px;
}

.custom-header {
  justify-content: space-evenly;
}

.header h1 {
  text-align: center;
  margin: 0;
  height: 24px;
  margin-top: -5px;
  padding: 0;
}

.hyphen {
  font-size: 20px;
  font-weight: bold;
}
</style>