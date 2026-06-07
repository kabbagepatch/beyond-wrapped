<template>
  <div class="dropdown-container">
    <button @click="toggleDropdown"><h1 :class="'title' + (isOpen ? ' title-selected' : '')">{{ title }}</h1></button>
    <ul v-if="isOpen" class="dropdown-menu">
      <li v-for="o in options" :key="o" @click="selectOption(o)">
        {{ o }}
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const props = defineProps<{
  title: string,
  options: string[],
  selectOption: (o: string) => void,
}>();

const isOpen = ref(false);
const toggleDropdown = () => {
  isOpen.value = !isOpen.value;
};
const selectOption = (option: string) => {
  props.selectOption(option);
  isOpen.value = false;
}

</script>

<style scoped>
button {
  text-align: left;
  height: 24px;
}

.title-selected {
  background-color: var(--background-color-dark);
}

.dropdown-container {
  position: relative;
  display: inline-block;
}

.dropdown-menu {
  position: absolute;
  background-color: var(--background-color-dark);
  min-width: 92px;
  max-height: 160px;
  overflow-y: scroll;
  box-shadow: 0px 8px 16px 0px rgba(0,0,0,0.2);
  z-index: 1;
  list-style-type: none;
  padding: 0;
  padding-top: 8px;
  margin: 0;
  scrollbar-width: thin;
}

.dropdown-menu li {
  padding: 6px 12px;
  cursor: pointer;
  font-family: Pixels, Inter, Avenir, Helvetica, Arial, sans-serif;
  text-shadow: -1.5px -1.5px 0 var(--text-outline), 1.5px -1.5px 0 var(--text-outline), -1.5px 1.5px 0 var(--text-outline), 1.5px 1.5px 0 var(--text-outline);
  color: var(--text-color);
  font-size: 24px;
}
</style>