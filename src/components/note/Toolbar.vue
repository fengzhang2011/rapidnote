<template>
  <div>
    <button class="tbbtn btnbold">B</button>
    <button class="tbbtn btnbold btnitalic">I</button>
    <button class="tbbtn btnbold btnunderline">U</button>
    <button class="tbbtn btnbold">"</button>
    <button class="tbbtn btnbold">
      <svg width="15px" height="15px" viewBox="0 -3 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
        <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
        <g id="SVGRepo_iconCarrier">
          <path
            d="M8 6L21 6.00078M8 12L21 12.0008M8 18L21 18.0007M3 6.5H4V5.5H3V6.5ZM3 12.5H4V11.5H3V12.5ZM3 18.5H4V17.5H3V18.5Z"
            stroke="#000000" stroke-width="1.3679999999999999" stroke-linecap="round" stroke-linejoin="round"></path>
        </g>
      </svg>
    </button>
    <button class="tbbtn btnbold">
      <svg width="15px" height="15px" viewBox="0 -3 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
        <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
        <g id="SVGRepo_iconCarrier">
          <path
            d="M10 6L21 6.00066M10 12L21 12.0007M10 18L21 18.0007M3 5L5 4V10M5 10H3M5 10H7M7 20H3L6.41274 17.0139C6.78593 16.6873 7 16.2156 7 15.7197C7 14.7699 6.23008 14 5.28033 14H5C4.06808 14 3.28503 14.6374 3.06301 15.5"
            stroke="#000000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path>
        </g>
      </svg>
    </button>
    <button class="tbbtn btnbold" style="margin-top: 2px;">
      <svg width="15px" height="15px" viewBox="0 -3 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <g id="SVGRepo_bgCarrier" stroke-width="0"></g>
        <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g>
        <g id="SVGRepo_iconCarrier">
          <path d="M7 8L3 11.6923L7 16M17 8L21 11.6923L17 16M14 4L10 20" stroke="#000000" stroke-width="2"
            stroke-linecap="round" stroke-linejoin="round"></path>
        </g>
      </svg>
    </button>
    <SelectBox :options="fontSizes"></SelectBox>
    <SelectBox :options="textSizes"></SelectBox>
    <SelectBox :options="fontFamilies"></SelectBox>
  </div>
</template> 

<style scoped>
.tbbtn {
  border: none;
  border-radius: 1px;
  box-shadow: none;
  width: 25px;
  height: 25px;
  margin: 0;
  padding: 0;
  line-height: 28px;
  text-align: center;
  font-size: 16px;
  font-family: 'Courier New', Courier, monospace;
  background: #f6f6f6;
}

.tbbtn:hover {
  border: none;
  background: #e7e7e7;
}

.btnbold {
  font-weight: bold;
}

.btnitalic {
  font-style: italic;
}

.btnunderline {
  text-decoration: underline;
}
</style>

<script setup>
import { ref } from "vue";
const fontSizes = ref([
  { label: 'Small', value: 'small' },
  { label: 'Normal', value: 'normal' },
  { label: 'Large', value: 'value' },
  { label: 'Huge', value: 'huge' },
]);

const textSizes = ref([
  { label: 'Heading 1', value: 'heading1' },
  { label: 'Heading 2', value: 'heading2' },
  { label: 'Heading 3', value: 'heading3' },
  { label: 'Heading 4', value: 'heading4' },
  { label: 'Heading 5', value: 'heading5' },
  { label: 'Body', value: 'body' },
]);

const fontFamilies = ref([
  { label: 'Sans Serif', value: 'sansserif' },
  { label: 'Serif', value: 'serif' },
  { label: 'Monospace', value: 'monospace' },
]);

</script>