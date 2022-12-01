<script setup lang="ts">
import leftList from "./components/LeftList.vue";
import rightList from "./components/RightList.vue";
</script>

<template>
  <div class="flex w-[700px] m-auto justify-between mt-10">
    <leftList msg="leftList" />
    <rightList msg="rightList" />
  </div>
</template>

<style scoped></style>
