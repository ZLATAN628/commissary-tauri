import { createRouter, createWebHashHistory } from 'vue-router'
import Main from '../views/Main.vue'
import { appLocalDataDir } from '@tauri-apps/api/path';

const routes = [
    {
        path: "/",
        name: 'Login',
        component: () =>
            import('../views/Login.vue')
    },
    {
        path: '/Main',
        name: 'Main',
        component: Main
    },
    {
        path: '/BackMain/:productList:amount:num',
        name: 'BackMain',
        component: Main
    },
    {
        path: '/Product',
        name: 'Product',
        component: () =>
            import('../views/Product.vue')
    },
    {
        path: '/Qrcode/:productList:amount:num',
        name: 'Qrcode',
        component: () =>
            import('../views/Qrcode.vue')
    },
    {
        path: '/History/:name',
        name: 'History',
        component: () =>
            import('../views/History.vue')
    },
    {
        path: '/ShoppingList',
        name: 'ShoppingList',
        component: () =>
            import('../views/ShoppingList.vue')
    }
]

let dir = await appLocalDataDir();

const router = createRouter({

    //history模式：createWebHistory , hash模式：createWebHashHistory
    history: createWebHashHistory(),
    routes
})

export default router