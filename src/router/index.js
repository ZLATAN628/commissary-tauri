import { createRouter, createWebHashHistory } from 'vue-router'
import Main from '../views/Main.vue'
import { appLocalDataDir } from '@tauri-apps/api/path';

const routes = [
    // { path: "/", redirect: "/index" },
    {
        path: '/:isSettle*',
        name: 'Main',
        component: Main
    },
    {
        path: '/Product',
        name: 'Product',
        component: () =>
            import('../views/Product.vue')
    },
    {
        path: '/Qrcode/:amount',
        name: 'Qrcode',
        component: () =>
            import('../views/Qrcode.vue')
    }
]

let dir = await appLocalDataDir();

const router = createRouter({

    //history模式：createWebHistory , hash模式：createWebHashHistory
    history: createWebHashHistory(),
    routes
})

export default router