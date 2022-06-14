import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import VueLazyLoad from 'vue3-lazyload'

createApp(App).use(router).use(VueLazyLoad, {
  loading: require('@/assets/XOsX.gif')
}).mount('#app')
