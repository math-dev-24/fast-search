import { createRouter, createWebHistory } from 'vue-router'
import Home from './views/Home.vue'
import Statistique from './views/Statistique.vue'

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            name: 'home',
            meta: { labelKey: "header.home" },
            component: Home
        },
        {
            path: '/statistique',
            name: 'statistics',
            meta: { labelKey: "header.statistics" },
            component: Statistique
        }
    ]
})

export default router