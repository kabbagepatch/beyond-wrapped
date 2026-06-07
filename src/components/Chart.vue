<template>
  <div class="chart-container">
    <div ref="chart">
      
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

const chart = ref(null)

onMounted(() => {
  const data = [10, 120, 30, 80]

  const width = 200
  const height = 100
})
</script>

<style scoped>
.chart-container {
  padding: 10px;
  padding-top: 0;
}
</style>
