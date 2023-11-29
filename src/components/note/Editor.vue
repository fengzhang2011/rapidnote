<template>
  <div>
    <TextEditor class="rapid-note-editor-container" @update="handleUpdated"></TextEditor>
  </div>
</template>

<style scoped>
.rapid-note-editor-container {
  /* border: 1px solid red; */
  width: 100%;
  height: 100%;
}
</style>

<script setup>
const emit = defineEmits(['inFocus', 'submit'])
function handleUpdated(text) {
  emit('update', text)
}
</script>

