<template>
  <div class="container">
    <Header :rightButtonClick="goToSettings" />
    <button>
      <router-link :to="`/year/${toYear}`">
        <title-card
          title="Yearly"
          iconName="year"
          :subtitles="['View your Top Tracks, Artists and Albums by year']"
        />
      </router-link>
    </button>
    <button>
      <router-link :to="`/year/${toYear}/${toMonth}`">
        <title-card
          title="Monthly"
          iconName="month"
          :subtitles="['Dive Deeper into your Top Tracks, Artists and Albums by month']"
        />
      </router-link>
    </button>
    <button>
      <router-link :to="`/custom?from=${fromMonth}-${fromYear}&to=${toMonth}-${toYear}`">
        <title-card
          title="Custom"
          iconName="calendar"
          :subtitles="['Pick a custom date range to view your stats']"
        />
      </router-link>
    </button>
    <button>
      <router-link to="/search">
        <title-card
          title="Search"
          iconName="calendar"
          :subtitles="['Search for your favorite tracks or artists']"
        />
      </router-link>
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import TitleCard from "../components/TitleCard.vue";
import Header from "../Header.vue";
import { invoke } from "@tauri-apps/api/core";

const router = useRouter();
const goToSettings = () => {
  router.push('/settings');
}

const fromMonth = ref(1);
const fromYear = ref(2020);
const toMonth = ref(12);
const toYear = ref(2025);
invoke('get_bounds').then((r: any) => {
  const minTS = r.min_timestamp;
  const minDate = new Date(minTS);
  const maxTS = r.max_timestamp;
  const maxDate = new Date(maxTS);

  fromMonth.value = minDate.getMonth() + 1;
  fromYear.value = minDate.getFullYear();
  toMonth.value = maxDate.getMonth() + 1;
  toYear.value = maxDate.getFullYear();
}).catch(e => {
  console.log(e);
});

</script>

<style scoped>

button {
  margin-top: 4px;
  margin-bottom: 12px;
}
</style>