import { createRouter, createWebHistory } from "vue-router";

const routes = [
  {
    path: "/",
    name: "top",
    component: async () => {
      return await import("@/pages/Top.vue");
    },
  },
  {
    path: "/signup",
    name: "signup",
    component: () => import("@/pages/Signup.vue"),
  },
  {
    path: "/signin",
    name: "signin",
    component: () => import("@/pages/Signin.vue"),
  },
  {
    path: "/registerprofile",
    name: "RegisterProfile",
    component: () => import("@/pages/RegisterProfile.vue"),
  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

export default router;
