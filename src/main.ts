import {createApp} from "vue";
import "./styles.css";
import {createRouter, createWebHistory, RouteRecordRaw} from "vue-router";
import Index from "./Views/Index.vue";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from "./App.vue";
import Data from "./Views/data.vue";

const routes: Array<RouteRecordRaw> = [
    {path: "/", component: Index},
    {path: "/data", component: Data},
]
const router = createRouter({
    history: createWebHistory(),
    routes
})
const app = createApp(App);
app.use(router);
app.use(ElementPlus)
app.mount("#app");
