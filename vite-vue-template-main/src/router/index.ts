import { createRouter, createWebHistory } from "vue-router";

const routes = [
  {
    path: "/",
    name: "top",
    component: async () => {
      return await import("@/pages/Top.vue");
    },
  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

export default router;
