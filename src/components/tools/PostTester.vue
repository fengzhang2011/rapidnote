<template>
  <div class="post-test-container">
    <h3>HTTP POST/GET Tester</h3>
    <div style="width: 500px; display: flex; justify-content: space-between; align-items: center;">
      <input id="url" @input="handleTextEdited()" placeholder="Please input URL." style="width: 380px;">
      <el-button type="primary" @click="handleSend()">Send</el-button>
    </div>
    <div style="width: 500px; display: flex; flex-direction: column; margin-top: 20px">
      <textarea id="request" @input="handleTextEdited()" placeholder="Please input request."
        style="width: 495px; height: 300px;"></textarea>
      <div id="response" style="width: 500px; height: 500px; border: 1px solid red;"></div>
    </div>
  </div>
</template>

<style scoped>
.post-test-container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
</style>

<script lang="ts" setup>
import { onMounted, ref } from 'vue';

// const url = ref('https://jsonplaceholder.typicode.com/posts');
// const data = { title: 'foo', body: 'bar', userId: 1 };
const url = ref('http://74.208.92.118:30010/command/system');
const data = { action: 'devices' };

const makePostRequest = async (url, data) => {
  try {
    const response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json;charset=UTF-8',
        Accept: 'application/json, text/plain, */*',
        'Accept-Encoding': 'gzip, deflate',
        'Accept-Language': 'en-US,en;q=0.9,de;q=0.8,ru;q=0.7,zh-CN;q=0.6,zh;q=0.5,zh-TW;q=0.4,ja;q=0.3',
        Authorization: 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXUyJ9.eyJleHBpcmUiOiIxNzI5NzQxMDczIiwiaXNzIjoiWGluc2VDaGlwIiwibG9naW4iOiJzdXBlcmFkbWluIiwidmVyc2lvbiI6IjEuMC4wIn0.l9GL6GHalh3vVa06yUW5gJKkaZJiPN1C90A23Vq3WVA',
        Connection: 'keep-alive',
        Host: '74.208.92.118:30010',
        Origin: 'http://localhost:30000',
        Referer: 'http://localhost:30000/',
        'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36'
      },
      body: JSON.stringify(data)
    });

    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }

    return await response.json();
  } catch (error) {
    console.error('Error making POST request:', error);
  }
}

const handleTextEdited = () => {
  url.value = document.getElementById('url')?.innerText || '';
};

const handleSend = async () => {
  // const res = await axios.get('https://herdwize.com');
  const resp = document.getElementById('response');
  if (resp) {
    makePostRequest(url.value, data)
      .then(responseData => resp.innerText = JSON.stringify(responseData))
      .catch(error => console.error(error));
    // fetch('https://herdwize.com')
    //   .then(response => response.json())
    //   .then(json => response.innerText = JSON.stringify(json))
    //   .catch(err => console.error('Error:', err));
    // fetch('https://jsonplaceholder.typicode.com/posts/1')
    //   .then(response => resp.innerText = response.toString())
    //   .catch(err => console.error('Error:', err));
  }
};

onMounted(() => { });
</script>