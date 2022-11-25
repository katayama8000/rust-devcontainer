<script setup lang="ts">
import { computed } from "@vue/reactivity";
import { useStore } from "vuex";
import { key } from "../store";

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
  <div>
    <div
      v-if="leftItems.length > 0"
      class="w-[300px] border-navy text-center pb-6"
    >
      <div v-for="item in leftItems" :key="item" class="mt-4">
        <div class="p-1 navy_underline">
          <span class="text-[16px]">
            {{ item }}
          </span>
        </div>
      </div>
    </div>
    <div class="m-auto w-[150px] mt-[30px]">
      <button
        @click="moveLeftToRight"
        class="w-[150px] text-center h-[40px] bg-color-navy text-white"
      >
        右に移動
      </button>
    </div>
  </div>
</template>

<style scoped></style>
