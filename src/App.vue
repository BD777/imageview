<template>
  <!-- <div>{{ data.a }}</div>
  <n-button>123</n-button>
  <img alt="Vue logo" src="./assets/logo.png">
  <HelloWorld msg="Welcome to Your Vue.js App"/> -->
  <router-view></router-view>
</template>

<script>
import { reactive } from 'vue'
// import { invoke } from '@tauri-apps/api/tauri'
import { dialog } from '@tauri-apps/api'
import $backend from './backend'

export default {
  name: 'App',
  components: {
  },
  setup () {
    const data = reactive({
      a: 1
    })

    console.log('dialog', dialog)
    console.log('backend', $backend)
    // dialog.open({
    //   title: '选择图片目录',
    //   directory: true
    // }).then(res => {
    //   console.log('选择目录结果', res)
    // })

    $backend.initTable().then(res => {
      console.log('initTable', res)
    })

    $backend.getImagesMetaList(1, 10).then(res => {
      console.log('getImagesMetaList', res)
    })

    // invoke('hello', {
    //   a: 1,
    //   b: '23'
    // }).then(res => {
    //   console.log('succ', res)
    // }, err => {
    //   console.error('error', err)
    // })
    // $backend.hello(1, '23').then(res => {
    //   console.log('succ', res)
    // }, err => {
    //   console.error('error', err)
    // })

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
  color: #2c3e50;
}
</style>
