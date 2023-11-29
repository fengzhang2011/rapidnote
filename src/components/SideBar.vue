<template>
  <div style="width: 200px; height: 100%; background: #3c3c3c;">
    <div class="menu">
      <ul>
        <li v-for="(item, index) in menus" :keys="index" :id="item.id" class="item">
          <input type="checkbox" />
          <a class="btn">{{ item.label }}</a>
          <ul v-if="item.submenus && item.submenus.length" class="smenu">
            <li v-for="(sitem, sidx) in item.submenus" :keys="sidx" :id="sitem.id">
              <div @click="handleMenuClick(sitem.id)">{{ sitem.label }}</div>
            </li>
          </ul>
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
ul {
  list-style: none;
  list-style-type: none;
  list-style-position: inside;
  padding: 0;
  margin: 0;
}


.menu {
  width: 200px;
  overflow: hidden;
}

.item {
  /* border-top: 1px solid #f1f1f1; */
  position: relative;
  overflow: hidden;
}

.item input[type=checkbox] {
  width: 100%;
  height: 100%;
  position: absolute;
  z-index: 1;
  opacity: 0;
  cursor: pointer;
  background: #3c3c3c;
}

.item input[type=checkbox]:checked~.smenu {
  max-height: 100em;
}

.btn {
  display: block;
  padding: 10px;
  background: #3c3c3c;
  color: #f0f0f0;
  position: relative;
}

.btn::before {
  content: "";
  position: absolute;
  width: 14px;
  height: 14px;
  background: #3c3c3c;
  left: 20px;
  bottom: -7px;
  transform: rotate(45deg);
}

.btn i {
  margin-right: 10px;
}

.smenu {
  background: #2a2a2a;
  overflow: hidden;
  transition: max-height 0.1s;
  max-height: 0;
  position: relative;
  z-index: 10;
}

.smenu li {
  display: block;
  padding: 3px 0 3px 30px;
  color: #f0f0f0;
  font-size: 14px;
  margin: 0;
  position: relative;
  cursor: pointer;
}

.smenu li:before {
  content: "";
  position: absolute;
  width: 6px;
  height: 100%;
  background: #ee1111;
  left: 0;
  top: 0;
  opacity: 0;
  transition: 0.3s;
}

.smenu li:hover {
  color: #ee1111;
}

.smenu li:hover:before {
  opacity: 1;
}
</style>
<script lang="ts" setup>
import { reactive } from 'vue'
const emit = defineEmits<{
  clicked: [menu: String];
}>();


const menus = reactive([
  {
    id: 'editor',
    label: 'Rapid Note',
    submenus: [
      { id: 'note', label: 'Note Editor' },
    ]
  },
  {
    id: 'tomato-timer',
    label: 'Tomato',
    submenus: [
      { id: 'tomato', label: 'Tomato' },
    ]
  },
  {
    id: 'settings',
    label: 'Settings',
    submenus: [
      { id: 'password', label: 'Password' },
      { id: 'language', label: 'Language' },
    ]
  },
  {
    id: 'signout',
    label: 'Sign-Out',
    submenus: [],
  }
]);

const handleMenuClick = (id: String) => {
  emit("clicked", id);
};

</script>

