<script setup lang="ts">
import { computed } from "@vue/reactivity";
import { useStore } from "vuex";
import { key } from "./store";
import leftList from "./components/LeftList.vue";
import rightList from "./components/RightList.vue";

const store = useStore(key);
</script>

<template>
  <div class="flex w-max-[700px]">
    <div class="w-1/2">
      <leftList msg="leftList" />
    </div>
    <div class="w-1/2">
      <rightList msg="rightList" />
    </div>
  </div>
</template>

<style scoped></style>
