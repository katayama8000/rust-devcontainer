<script setup lang="ts">
import { ref } from "vue";
import { computed } from "@vue/reactivity";
import { useStore } from "vuex";
import { key } from "../store";
defineProps<{ msg: string }>();
const store = useStore(key);
//leftItems
const leftItems = computed<string[]>(() => {
  return store.state.leftItems;
});

const moveLeftToRight = (): void => {
  store.commit("moveLeftToRight");
};
</script>

<template>
  <div class="w-1/2"></div>
    <h1>{{ msg }}</h1>
    <h1>left</h1>
    <div v-for="item in leftItems" :key="item">
      {{ item }}
    </div>
    <button @click="moveLeftToRight" class="bg-purple-200">右に移動</button>
  </div>
</template>

<style scoped></style>
