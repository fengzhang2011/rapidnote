<template>
  <div class="banner-container">
    <div class="date-time">{{ formatDateTime(currentTime) }}</div>
    <div class="date-time" style="display: flex">
      <div style="margin-right: 20px; color: green">
        {{ formatTomatoTime(tomato) }}
      </div>
      <div style="display: flex; align-items: center">
        <el-button type="success" size="small" @click="handleRestart">Restart</el-button>
      </div>
      <div>{{ haha }}</div>
      <div style="margin: 0 20px; color: red">
        {{ formatTomatoTime(rest) }}
      </div>
      <div style="display: flex; align-items: center">
        <el-button type="danger" size="small" @click="handleRest">Rest</el-button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.banner-container {
  height: 50px;
  line-height: 50px;
  padding: 0 10px;
  display: flex;
  justify-content: space-between;
}

.date-time {
  font-size: 2em;
}
</style>
<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const currentTime = ref(new Date());
const tomato = ref(25 * 60); // seconds
const rest = ref(0 * 60); // seconds
const haha = ref("a");

const padNumber = (num: Number, size: number) => {
  let output = num.toString();
  while (output.length < size) output = "0" + output;
  return output;
};

const formatDateTime = (date: Date) => {
  const year = date.getFullYear();
  const month = padNumber(date.getMonth() + 1, 2);
  const day = padNumber(date.getDate(), 2);
  const hour = padNumber(date.getHours(), 2);
  const minute = padNumber(date.getMinutes(), 2);
  const second = padNumber(date.getSeconds(), 2);
  return `${year}-${month}-${day} ${hour}:${minute}:${second}`;
};

const formatTomatoTime = (value: number) => {
  const minutes = Math.floor(value / 60);
  const seconds = padNumber(value % 60, 2);
  return `${minutes}:${seconds}`;
};

const handleRestart = () => {
  tomato.value = 25 * 60;
  rest.value = 0;
};

const handleRest = () => {
  tomato.value = 0;
  rest.value = 5 * 60;
};

async function getTomatoClockStatus() {
  // console.log(invoke);
  // const clock = await invoke("get_tomato_clock");
  // console.log("Tomato Clock:", clock);
  // haha.value = clock.clock_type;
  // haha.value = await invoke("haha", {});
}

onMounted(() => {
  setInterval(() => {
    currentTime.value = new Date();
    if (tomato.value > 0) {
      tomato.value--;
      if (tomato.value === 0) {
        rest.value = 5 * 60;
      }
    }
    if (rest.value > 0) {
      rest.value--;
      if (rest.value === 0) {
        tomato.value = 25 * 60;
      }
    }
  }, 1000);
  getTomatoClockStatus();
});
</script>
