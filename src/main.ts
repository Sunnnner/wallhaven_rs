import { createApp } from 'vue';
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';
import App from './App.vue';
import router from './router';

const app = createApp(App)


document.oncontextmenu = function () {
    return false;
}

app.use(ElementPlus)
app.use(router)
app.mount('#app')