<template>
  <card>
    <div class="container">
      <div class="title-container">
        <img v-if="iconName" class="icon" :src="`/src/assets/icons/${iconName}.png`" />
        <h1 class="title">{{ title }}</h1>
      </div>
      <div v-if="subtitles" class="subtitle">
        <div v-for="(subtitle) in subtitles" >{{ subtitle }}</div>
      </div>
    </div>
  </card>
</template>

<script setup lang="ts">
import Card from './Card.vue';

defineProps<{
  title: string;
  subtitles?: string[];
  iconName?: string;
}>();

</script>

<style scoped>
.container {
  width: 100%;
  height: 124px;
  display: flex;
  justify-content: center;
  align-items: center;
}

.title-container {
  display: flex;
  align-items: center;
  gap: 12px;
}

.title-container .icon {
  display: block;
  width: 40px;
  height: 40px;
}

.header .title {
  margin: 0;
}

.title {
  font-size: 32px;
  margin: 0;
}

.subtitle {
  margin-top: 10px;
  font-size: 14px;
  font-family: Helvetica, Arial, sans-serif;
  text-shadow: none;
  color: var(--background-color);
}
</style>