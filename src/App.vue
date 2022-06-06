<template>
  <div>{{ data.a }}</div>
  <img alt="Vue logo" src="./assets/logo.png">
  <HelloWorld msg="Welcome to Your Vue.js App"/>
</template>

<script>
import { reactive } from 'vue'
import HelloWorld from './components/HelloWorld.vue'
import { invoke } from '@tauri-apps/api/tauri'
import { dialog } from '@tauri-apps/api'

export default {
  name: 'App',
  components: {
    HelloWorld
  },
  setup () {
    const data = reactive({
      a: 1
    })

    console.log('dialog', dialog)
    dialog.open({
      title: '选择图片目录',
      directory: true
    }).then(res => {
      console.log('选择目录结果', res)
    })

    invoke('hello', {
      a: 1,
      b: '23'
    }).then(res => {
      console.log('succ', res)
    }, err => {
      console.error('error', err)
    })

    return {
      data
    }
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
