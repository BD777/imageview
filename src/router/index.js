const VueRouter = require('vue-router')

console.log(VueRouter)

const routes = [
  { path: '/', name: 'my-lib', component: require('@/views/MyLibrary').default }
]

const router = VueRouter.createRouter({
  history: VueRouter.createWebHashHistory(),
  routes
})

export default router
