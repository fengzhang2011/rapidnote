<template>
  <div class="textarea-container">
    <div class="line-numbers" id="lineNumbers"></div>
    <textarea id="textarea" class="editor" @input="handleTextEdited()" @scroll="syncScroll()"></textarea>
  </div>
</template>

<style scoped>
.editor {
  width: calc(100% - 68px);
  height: 100%;
}

.textarea-container {
  position: relative;
}

.line-numbers {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 50px;
  overflow: hidden;
  background-color: #eee;
  text-align: right;
  padding-right: 10px;
  border-right: 1px solid #ddd;
  user-select: none;
  font-family: monospace;
  font-size: 16px;
  line-height: 20px;
  padding-top: 8px;
}

textarea {
  position: absolute;
  left: 50px;
  top: 0;
  bottom: 0;
  width: calc(100% - 50px);
  border: none;
  resize: none;
  font-family: monospace;
  font-size: 16px;
  line-height: 20px;
  padding: 8px;
  outline: none;
}
</style>

<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { info } from "tauri-plugin-log-api";

const textarea = ref("");
const output = ref("");
const emit = defineEmits(['inFocus', 'submit'])

const handleTextEdited = async function () {
  updateLineNumbers();
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const text = document.getElementById('textarea').value
  info('ahhasdf')
  output.value = await invoke("markdown2html", { text });
  // document.getElementById('output').innerHTML = output.value;
  emit('update', output.value)
}

const updateLineNumbers = () => {
  const textArea = document.getElementById('textarea');
  const lineNumbers = document.getElementById('lineNumbers');
  const text = textArea.value;
  const lineHeight = parseInt(window.getComputedStyle(textArea).lineHeight);
  const lines = (textArea.scrollHeight - parseInt(window.getComputedStyle(textArea).paddingTop) - parseInt(window.getComputedStyle(textArea).paddingBottom)) / lineHeight;
  let lineNumberHtml = '';
  for (let i = 1; i <= lines; i++) {
    lineNumberHtml += `${i}<br>`;
  }
  lineNumbers.innerHTML = lineNumberHtml;
}

const syncScroll = () => {
  const textArea = document.getElementById('textarea');
  const lineNumbers = document.getElementById('lineNumbers');
  lineNumbers.scrollTop = textArea.scrollTop;
}

</script>

