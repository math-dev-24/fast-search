import { createRouter, createWebHistory } from 'vue-router'
import Home from './views/Home.vue'
import Statistique from './views/Statistique.vue'

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            name: 'Accueil',
            component: Home
        },
        {
            path: '/statistique',
            name: 'Statistique',
            component: Statistique
        }
    ]
})

export default router