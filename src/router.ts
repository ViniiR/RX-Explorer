import { createRouter, createWebHistory } from "vue-router";

const routes = [
    {
        path: "/",
        name: "Home",
        redirect: '/disks'
    },
    {
        path: "/dir:dirName",
        name: "directory",
        component: () => import("./components/Directory.vue"),
    },
    {
        path: "/disks",
        name: "disks",
        component: () => import("./components/Disks.vue"),
    },
    {
        path: "/text:fileName",
        name: "text",
        component: () => import("./components/TextEditor.vue"),
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

// router.beforeEach((to, from, next) => {
//     if (to.path === '/') {
//         next(false);
//     } else {
//         next();
//     }
// });

export default router;
