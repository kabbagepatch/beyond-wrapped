<template>
  <div class="header">
    <img class="icon" :src="`/src/assets/icons/${icon || defaultIcon}.png`" />
    <h1 class="title">{{ title || defaultTitle }}</h1>
    <button class="right-button" @click="onRightButtonClick">
      <img v-if="rightIcon" class="icon" :src="`/src/assets/icons/${rightIcon}.png`" />
    </button>
  </div>
</template>

<script setup lang="js">
import { ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const props = defineProps({
  icon: {
    type: String,
  },
  title: {
    type: String,
  },
  rightButtonClick: {
    type: Function,
  }
})

const router = useRouter();
const route = useRoute();

const defaultIcon = ref('note-transparent')
const defaultTitle = ref('Beyond Wrapped');
const rightIcon = ref('back')
const routeParts = route.fullPath.split('/');
if (routeParts.length > 1) {
  switch(routeParts[1]) {
    case 'player':
      defaultIcon.value = 'note-transparent'
      defaultTitle.value = 'Now Playing';
      break;
    case '':
      defaultIcon.value = 'award-transparent'
      defaultTitle.value = 'Beyond Wrapped';
      rightIcon.value = 'settings'
      break;
    case 'settings':
      rightIcon.value = 'back'
      break;
    default: 
      defaultIcon.value = 'note-transparent'
      rightIcon.value = 'back'
      break;
  }
}

const onRightButtonClick = () => {
  if (props.rightButtonClick) {
    props.rightButtonClick();
  } else {
    router.back();
  }
}

</script>

<style scoped>
.header {
  min-width: var(--width);
  display: flex;
  align-items: center;
  margin: -20px;
  margin-bottom: 10px;
  background-color: var(--background-color-dark);
  padding: 15px 20px;
}

.header .icon {
  display: block;
  margin-right: 10px;
  width: 30px;
  height: 30px;
}

.header .title {
  margin: 0;
  font-size: 30px;
  color: var(--text-color);
}

.header .right-button {
  width: 37px;
  height: 30px;
  border-radius: 5px;
}

.header .right-button-background {
  background-color: var(--primary-color);
}

.right-button .icon {
  width: 25px;
  height: 25px;
  margin: 1px;
}
</style>

