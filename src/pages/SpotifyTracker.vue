<template>
  <div class="home">
    <Header />
    <button>
      <router-link to="/tracker/basic">
        <title-card
          title="Basic Stats"
          iconName="stats"
          :subtitles="[
            'Get Top Tracks and Artists from up to the last year.',
            'Requires a connection to Spotify'
          ]"
        />
      </router-link>
    </button>
    <button>
      <router-link to="/tracker/extended">
        <title-card
          title="Extended Stats"
          iconName="cells"
          :subtitles="[
            'Explore your full Spotify Music History.',
            'Requires your extended history from Spotify'
          ]"
        />
      </router-link>
    </button>
  </div>
</template>

<script setup lang="ts">
import Header from "../Header.vue";
import TitleCard from "../components/TitleCard.vue";
</script>

<style scoped>
button {
  margin-top: 16px;
}

</style>