<script setup lang="ts">
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
  <div>
    <div class="w-[300px] border-navy text-center">
      <div v-for="item in rightItems" :key="item">
        <div class="p-1">
          {{ item }}
        </div>
      </div>
    </div>
    <button
      @click="moveRightToLeft"
      class="bg-purple-200 w-[150px] text-center m-auto"
    >
      左に移動
    </button>
  </div>
</template>

<style scoped></style>
