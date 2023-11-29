<template>
  <div style="display: flex;">
    <NotesList></NotesList>
    <div style="display: flex; flex-direction: column; height: 100%; width: 100%;">
      <div style="display: flex; justify-content: space-between; height: 100%;">
        <Rendered :html="output" style="width: 50%; height: 100%;"></Rendered>
        <div style="display: flex; flex-direction: column; height: 100%; width: 50%;">
          <Toolbar></Toolbar>
          <div style="width: 100%; height: 5px;"></div>
          <Editor @update="handleUpdated" style="width: 100%; height: 100%;"></Editor>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";

const output = ref("");

const handleUpdated = (text) => {
  output.value = text;
};

</script>