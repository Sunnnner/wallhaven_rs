import {createWebHistory, createRouter} from 'vue-router'

import Home from '../components/Home/index.vue';
import Top from '../components/Top/index.vue';
import Config from '../components/Config/index.vue';


const routes = [
    {
        path: '/',
        redirect: '/index',
    },
    {
        path: '/index',
        name: 'index',
        component: Home,
        children: [
            {
                path: 'top',
                name: 'top',
                component: Top,
            },
        ]
    },
    {
        path: '/config',
        name: 'config',
        component: Config,
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router;
