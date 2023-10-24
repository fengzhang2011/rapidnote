<template>
  <div>
    <el-input v-model="textarea" :rows="30" type="textarea" placeholder="Please input" @input="handleTextEdited" />
  </div>
</template>

<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const textarea = ref("");
const output = ref("");
const emit = defineEmits(['inFocus', 'submit'])

async function handleTextEdited() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  output.value = await invoke("markdown2html", { text: textarea.value });
  // document.getElementById('output').innerHTML = output.value;
  emit('update', output.value)
}

</script>

