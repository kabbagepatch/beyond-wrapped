<template>
  <div class="container">
    <Header :rightButtonClick="goToSettings" />
    <div class="description">
      <p>
        Spotify Wrapped feels so limited. Get a better insight into your listening history here.
        Request your Extended Listening History from Spotify <a href="https://www.spotify.com/us/account/privacy/" target="_blank">here</a>.
        Then head over to the Settings and upload the zip file you get from Spotify. And check out all the stats below.
      </p>
    </div>
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
.header {
  margin-bottom: 10px;
}

.description {
  padding: 0 4px;
  margin-bottom: 10px;
}

a {
  color: hsl(227, 96%, 69%);
}

button {
  margin-top: 4px;
  margin-bottom: 12px;
}
</style>