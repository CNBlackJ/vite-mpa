import { createWebHistory, createRouter } from 'vue-router'

import Layout from '@/views/Layout/index.vue'

import Home from '@/views/Home.vue'
import Recommend from '@/views/Recommend.vue'

const routes = [
  {
    path: '',
    component: Layout,
    children: [
      {
        path: '/home',
        name: 'Home',
        component: Home,
      },
      {
          path: '/recommend',
          name: 'Recommend',
          component: Recommend
      }   
    ]
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
