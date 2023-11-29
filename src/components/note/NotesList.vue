<template>
  <div style="height: 100%; width: 250px;">
    <ul id="notes">
      <li v-for="(item, index) in notes" :key="index" class="note-list">
        <div style="display: flex; flex-direction: column;">
          <div class="note" style="font-size: 1.1em; font-weight: bold; color: #333">{{ item.name }}</div>
          <div style="display: flex; font-size: 0.8em;">
            <div class="note">{{ item.folder }}</div>
            <div class="note">{{ item.modified }}</div>
          </div>
        </div>
      </li>
      <li class="note-list" style="height: 100px"></li>
    </ul>
  </div>
</template>

<style scoped>
.note-list {
  display: flex;
  flex-direction: column;
  user-select: none;
  /* border: 1px solid red; */
  padding: 5px;
}

.note {
  user-select: none;
  cursor: pointer;
}

ul {
  height: 100%;
  list-style-position: inside;
  padding-left: 0;
  overflow-y: scroll;
  scrollbar-width: none;
  /* Firefox */
  -ms-overflow-style: none;
  /* Internet Explorer 10+ */
}

ul::-webkit-scrollbar {
  /* Chrome, Safari 和 Opera */
  display: none;
}

li:hover {
  background: #ededed;
}

li.selected {
  background: #dfdfdf;
}
</style>

<script lang="ts" setup>
import { onMounted, reactive } from 'vue'
import { info } from "tauri-plugin-log-api";

const notes = reactive([
  { name: 'demo', tags: [], folder: 'folder', created: new Date(), modified: new Date() },
  { name: 'demo1', tags: [], folder: 'folder', created: new Date(), modified: new Date() },
  { name: 'demo2', tags: [], folder: 'folder', created: new Date(), modified: new Date() },
  { name: 'demo3', tags: [], folder: 'folder', created: new Date(), modified: new Date() },
  { name: 'demo4', tags: [], folder: 'folder', created: new Date(), modified: new Date() },
  { name: 'demo5', tags: [], folder: 'folder', created: new Date(), modified: new Date() },
  { name: 'demo6', tags: [], folder: 'folder', created: new Date(), modified: new Date() },
  { name: 'demo7', tags: [], folder: 'folder', created: new Date(), modified: new Date() },
  { name: 'demo8', tags: [], folder: 'folder', created: new Date(), modified: new Date() },
  { name: 'demo9', tags: [], folder: 'folder', created: new Date(), modified: new Date() },
  { name: 'demo10', tags: [], folder: 'folder', created: new Date(), modified: new Date() },
  { name: 'demo11', tags: [], folder: 'folder', created: new Date(), modified: new Date() },
]);

const handleNoteClicked = (event: any) => {
  let li = event.target;
  while (li && li.tagName !== 'LI') {
    li = li.parentNode;
  }
  if (li && li.tagName === 'LI') {
    const currentSelected = document.querySelector('li.selected');
    if (currentSelected) {
      currentSelected.classList.remove('selected');
    }
    li.classList.add('selected');
  }
};

onMounted(() => {
  const notes = document.getElementById("notes");
  if (notes) {
    notes.addEventListener("click", handleNoteClicked);
  }
});
</script>