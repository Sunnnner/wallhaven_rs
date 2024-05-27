import {createWebHistory, createRouter} from 'vue-router'

import Home from '../components/Home/index.vue';
import Index from '../components/Index/index.vue';
import Top from '../components/Top/index.vue';
import Hot from '../components/Hot/index.vue';
import random from '../components/Random/index.vue';
import Config from '../components/Config/index.vue';
import Tags from '../components/Tags/index.vue';


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
                path: 'latest',
                name: 'index',
                component: Index,
            },
            {
                path: 'top',
                name: 'top',
                component: Top,
            },
            {
                path: 'hot',
                name: 'hot',
                component: Hot,
            },
            {
                path: 'random',
                name: 'random',
                component: random,
            },
            {
                path: 'tags',
                name: 'tags',
                component: Tags,
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
