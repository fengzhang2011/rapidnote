<template>
  <div :style="mainContainerStyle">
    <Banner></Banner>
    <div style="display: flex; height: 100%;">
      <SideBar @clicked="handleMenuClick"></SideBar>
      <NoteMain v-if="page === 'note'" style="width: 100%;"></NoteMain>
      <TomatoMain v-else-if="page === 'tomato'" style="width: 100%;">
      </TomatoMain>
      <PostTester v-else-if="page === 'post'" style="width: 100%;"></PostTester>
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

<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { onMounted, ref } from "vue";
import { info } from "tauri-plugin-log-api";
import { appWindow } from "@tauri-apps/api/window";

const height = ref(768);
const width = ref(1024);
const mainContainerStyle = ref("");
const page = ref('note');

const handleMenuClick = (menu) => {
  page.value = menu;
};

onMounted(() => {
  appWindow.onResized(({ payload: size }) => {
    // info(JSON.stringify(size));
    height.value = size.height;
    width.value = size.width;
    mainContainerStyle.value = `margin: 0; padding: 0; width: ${width.value - 2}px; height: ${height.value - 2}px; overflow: hidden`;
    // info(editorStyle.value);
  })
});

</script>

