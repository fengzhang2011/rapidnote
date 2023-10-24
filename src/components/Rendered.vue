<template>
  <div>
    <div id="result"></div>
  </div>
</template>

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
