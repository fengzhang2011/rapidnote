<template>
  <div style="display: inline-block">
    <select v-if="options && options.length" class="select-box">
      <option v-for="(item, idx) in options" :key="idx" :value="item.value">{{ item.label }}</option>
    </select>
  </div>
</template> 

<style scoped>
.select-box {
  background: #f6f6f6 url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 18 18'%3E%3Cpolygon class='ql-stroke' points='7 11 9 13 11 11 7 11'%3E%3C/polygon%3E%3Cpolygon class='ql-stroke' points='7 7 9 5 11 7 7 7'%3E%3C/polygon%3E%3C/svg%3E") no-repeat top right;
  width: 100px;
  font-size: 14px;
  color: #545452;
  height: 25px;
  line-height: 25px;
  border: none;
  padding-left: 10px;
  -webkit-appearance: none;
  -moz-appearance: none;
  appearance: none;
  box-sizing: content-box;
  -moz-box-sizing: content-box;
  -webkit-box-sizing: content-box;
}

.select-box:hover {
  border-color: #888;
  background-color: #e7e7e7;
}

.select-box:focus {
  color: #222;
  outline: none;
}

.select-box option {
  font-weight: normal;
}
</style>

<script setup>
// import { watch } from 'vue';
const props = defineProps({
  options: [],
});
// watch(() => props.options, (first, second) => {
//   console.log(
//     "Watch props.selected function called with args:",
//     first,
//     second
//   );
// });
</script>
