<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";

import { ref } from "vue";
import { Close } from "@element-plus/icons-vue";

let id = 0;
const todosList = ref([]);
const text = ref("");

const onAddTodo = () => {
  todosList.value.push({
    id: id++,
    text: text.value,
  });
};

const onComplete = (index) => {
  todosList.value.splice(index, 1);
};

const handleEdited = () => {
  document.getElementById('msg').innerText = 'haha';
  todosList.value = [];
};
</script>

<template>
  <div style="display: flex;">
    <div class="container">
      <div id="msg"></div>
      <h1>Welcome to Tauri!</h1>
      <div class="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" class="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://vuejs.org/" target="_blank">
          <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

      <p>
        Recommended IDE setup:
        <a href="https://code.visualstudio.com/" target="_blank">VS Code</a>
        +
        <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
        +
        <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">Tauri</a>
        +
        <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">rust-analyzer</a>
      </p>

      <Greet @submit="handleEdited" />
    </div>
    <div class="todo-container">
      <h1>Todo List</h1>
      <div class="head">
        <el-input type="text" v-model="text"></el-input>
        <el-button @click="onAddTodo" type="primary">添 加</el-button>
      </div>
      <div class="list" v-if="todosList.length">
        <ul>
          <li v-for="(todo, index) in todosList" :key="todo.id">
            <span>{{ index + 1 }}. </span>
            <span>{{ todo.text }}</span>
            <el-icon :size="16" color="gray" @click="onComplete(index)" style="cursor: pointer;">
              <Close />
            </el-icon>
          </li>
        </ul>
      </div>
    </div>
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
