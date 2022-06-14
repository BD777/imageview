const VueRouter = require('vue-router')

console.log(VueRouter)

const routes = [
  { path: '/', name: 'my-lib', component: require('@/views/MyLibrary').default },
  { path: '/browse', name: 'browse', component: require('@/views/Browse').default }
]

const router = VueRouter.createRouter({
  history: VueRouter.createWebHashHistory(),
  routes
})

export default router
