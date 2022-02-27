import {createWebHistory, createRouter} from 'vue-router';

import Home from "@/views/Home.vue"
import Coach from "@/views/Coach.vue"
import Progress from "@/views/Progress.vue"

const routes = [{
    path: "/",
    name: "Home",
    component: Home
}, {
    path: "/:user/coach",
    name: "Coach",
    component: Coach
}, {
    path: "/:user/progress",
    name: "Progress",
    component: Progress
}]

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;