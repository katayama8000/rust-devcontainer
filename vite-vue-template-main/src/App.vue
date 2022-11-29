<script setup lang="ts">
import TheHeader from "@/components/TheHeader.vue";
</script>

<template>
  <!-- <TheHeader class="onlyPC" /> -->
  <nav>
    <ul>
      <li><router-link to="/">Home</router-link></li>
      <li><router-link to="/signup">Signup</router-link></li>
      <li><router-link to="/signin">Signin</router-link></li>
    </ul>
  </nav>
  <main>
    <RouterView class="contents" />
  </main>
</template>

<style lang="scss" scoped></style>
