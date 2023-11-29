<template>
  <div>
    <div id="result" class="rapid-note-rendered-container"></div>
  </div>
</template>

<style scoped>
.rapid-note-rendered-container {
  /* border: 1px solid red; */
  width: 100%;
  height: 100%;
  background: white;
}
</style>

<script setup>
import { watch } from 'vue';

const props = defineProps({
  html: "",
});

watch(() => props.html, (first, second) => {
  console.log(
    "Watch props.selected function called with args:",
    first,
    second
  );
  document.getElementById('result').innerHTML = second;
});
</script>
