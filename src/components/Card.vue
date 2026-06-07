<template>
  <div class="card">
    <div class="card-inner">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.card {
  width: var(--width);
  min-height: 156px;
  background-color: var(--primary-color);
  border-radius: 28px;
  filter: drop-shadow(0 0 0.15em var(--primary-color-shadow));
  padding: 1px;
  text-align: center;
}

.card-inner {
  margin: 3px;
  padding: 12px 16px;
  border: 1px solid var(--secondary-color);
  border-radius: 28px;
  min-height: 124px;
}
</style>