<script setup lang="ts">
import TheHeader from "@/components/TheHeader.vue";
</script>

<template>
  <TheHeader class="onlyPC" />
  <main>
    <RouterView class="contents" />
  </main>
</template>

<style lang="scss" scoped></style>
