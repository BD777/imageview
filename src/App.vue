<template>
  <!-- <div>{{ data.a }}</div>
  <n-button>123</n-button>
  <img alt="Vue logo" src="./assets/logo.png">
  <HelloWorld msg="Welcome to Your Vue.js App"/> -->

  <n-config-provider :theme="theme">
    <n-message-provider>
      <router-view></router-view>
    </n-message-provider>
    <n-global-style />
  </n-config-provider>
</template>

<script>
import { ref } from 'vue'
// import { invoke } from '@tauri-apps/api/tauri'
// import { dialog } from '@tauri-apps/api'
import $backend from './backend'
import { darkTheme, NConfigProvider, NGlobalStyle, NMessageProvider } from "naive-ui"

export default {
  name: 'App',
  components: {
    NConfigProvider,
    NGlobalStyle,
    NMessageProvider
  },
  setup () {
    $backend.initTable().then(res => {
      console.log('initTable', res)
    })

    let theme = ref(null)
    theme = darkTheme

    return {
      darkTheme,
      theme
    }
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}
</style>
