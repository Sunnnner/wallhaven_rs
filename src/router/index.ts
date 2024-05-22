import {createWebHistory, createRouter} from 'vue-router'

const routes = [
    {
        path: '/',
        name: 'home',
        component: () =>import("../components/Home/index.vue"),
        meta: {
            title: "首页"
        }
    },
    {
        path: '/top',
        name: 'top',
        component: () =>import("../components/Top/index.vue"),
        meta: {
            title: "置顶"
        }
    },
    {
        path: '/config',
        name: 'config',
        component: () =>import("../components/Config/index.vue"),
        meta: {
            title: "配置文件"
        }
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router;
