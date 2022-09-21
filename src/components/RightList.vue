<script setup lang="ts">
import { ref } from "vue";
import { computed } from "@vue/reactivity";
import { useStore } from "vuex";
import { key } from "../store";
const store = useStore(key);

defineProps<{ msg: string }>();
const moveLeftToRight = (): void => {
  store.commit("moveLeftToRight");
};
const moveRightToLeft = (): void => {
  store.commit("moveRightToLeft");
};

const rightItems = computed<string[]>(() => {
  return store.state.rightItems;
});
</script>

<template>
  <div class="w-1/2">
    <h1>{{ msg }}</h1>
    <h1>left</h1>
    <div v-for="item in rightItems" :key="item">
      {{ item }}
    </div>
    <button @click="moveRightToLeft" class="bg-purple-200">左に移動</button>
  </div>
</template>

<style scoped></style>
