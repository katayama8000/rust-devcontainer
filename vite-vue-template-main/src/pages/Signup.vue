<script setup lang="ts">
import { useStore } from "vuex";
import { key } from "../store";
const store = useStore(key);
const count = store.state.count;
</script>

<template>
  <h1>{{ count }}</h1>
</template>

<style lang="scss" scoped></style>
