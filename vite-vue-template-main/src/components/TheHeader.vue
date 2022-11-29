<script setup lang="ts">
import { ref } from "vue";

const text = ref("HEADER");
</script>

<template>
  <header>{{ text }}</header>
</template>

<style lang="scss" scoped>
header {
  position: fixed;
  z-index: 999;
  top: 0;
  width: 100%;
  padding: 20px;
  box-shadow: 0px 10px 10px -5px rgba(0, 0, 0, 0.5);
  background-color: $colorPurple;
}
</style>
