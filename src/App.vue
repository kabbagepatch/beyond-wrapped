<template>
  <main class="container">
    <RouterView :key="$route.fullPath" />
  </main>
</template>

<script setup lang="ts">
import { load } from '@tauri-apps/plugin-store';
import { invoke } from '@tauri-apps/api/core';

const invokeProcessing = async () => {
  const store = await load('store.json');
  const processed = await store.get('full-history-processed');
  if (!processed) {
    invoke('process_raw_history');
  }
}

invokeProcessing();

</script>

<style>
@font-face {
  font-family: "Pixels";
  src: url('./assets/fonts/Jersey10-Regular.ttf');
}
@font-face {
  font-family: "Bubbly";
  src: url('./assets/fonts/Atop-R99O3.ttf');
}

:root {
  font-family: Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  background-color: var(--background-color);
  color: var(--text-color);
  background-repeat: no-repeat;
  background-size: cover;
  padding: 10px;
  display: flex;
  justify-content: center;
  align-items: center;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;

  scrollbar-width: none;

  --width: 364px;

  --primary-color: hsl(39, 59%, 78%);
  --primary-color-shadow: hsl(39, 59%, 58%);
  --secondary-color: hsl(18, 71%, 27%);
  --tertiary-color: hsl(31, 51%, 34%);
  --background-color: hsl(26, 42%, 19%);
  --text-color: hsl(0, 0%, 100%);
  --text-outline: hsl(26, 42%, 19%);
}

 @media (min-width: 400px) {
  :root {
    --width: 390px;
  }
}

 @media (min-width: 460px) {
  :root {
    --width: 450px;
  }
}

.container {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

h1, button, .h1 {
  font-family: Pixels, Inter, Avenir, Helvetica, Arial, sans-serif;
  text-shadow: -1.5px -1.5px 0 var(--text-outline), 1.5px -1.5px 0 var(--text-outline), -1.5px 1.5px 0 var(--text-outline), 1.5px 1.5px 0 var(--text-outline);
  color: var(--text-color);
}

h1 {
  margin: 0;
  margin-bottom: 10px;
  font-size: 28px;
  width: 100%;
}

button {
  font-size: 24px;
  display: block;
  margin: 0;
  padding: 0;
  cursor: pointer;
  border: none;
  background: none;
  outline: none;
}

a {
  text-decoration: none;
  color: var(--background-color);
}
</style>