<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref } from "vue";

const output = ref("");

const handleUpdated = (text) => {
  output.value = text;
};
</script>

<template>
  <div style="display: flex; justify-content: space-between;">
    <Editor @update="handleUpdated" style="width: 50vw; height: 100vh;"></Editor>
    <Rendered :html="output" style="width: 50vw;  height: 100vh;"></Rendered>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
