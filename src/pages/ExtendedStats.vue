<template>
  <div class="container">
    <Header :rightButtonClick="goToSettings" />
    <button>
      <router-link to="/year/2025">
        <title-card
          title="Yearly"
          iconName="year"
          :subtitles="['View your Top Tracks, Artists and Albums by year']"
        />
      </router-link>
    </button>
    <button>
      <router-link to="/year/2025/12">
        <title-card
          title="Monthly"
          iconName="month"
          :subtitles="['Dive Deeper into your Top Tracks, Artists and Albums by month']"
        />
      </router-link>
    </button>
    <button>
      <router-link to="/custom?from=1-2020&to=12-2025">
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
import { useRouter } from "vue-router";
import TitleCard from "../components/TitleCard.vue";
import Header from "../Header.vue";

const router = useRouter();
const goToSettings = () => {
  router.push('/settings');
}
</script>

<style scoped>

button {
  margin-top: 4px;
  margin-bottom: 12px;
}
</style>