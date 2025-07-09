import { createRouter, createWebHistory } from 'vue-router'
import Home from './views/Home.vue'

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            name: 'home',
            component: Home
        },
        {
            path: '/statistique',
            name: 'Statistique',
            component: () => import('./views/Statistique.vue')
        }
    ]
})

export default router