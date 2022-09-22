<script setup lang="ts">
import { computed } from "@vue/reactivity";
import { useStore } from "vuex";
import { key } from "../store";
const store = useStore(key);

defineProps<{ msg: string }>();

const moveRightToLeft = (): void => {
  store.commit("moveRightToLeft");
};

const rightItems = computed<string[]>(() => {
  return store.state.rightItems;
});
</script>

<template>
  <div>
    <div
      v-if="rightItems.length > 0"
      class="w-[300px] border-navy text-center pb-6"
    >
      <div v-for="item in rightItems" :key="item" class="mt-2">
        <div class="p-1 navy_underline">
          <span class="font-bold text-xl">
            {{ item }}
          </span>
        </div>
      </div>
    </div>
    <div class="m-auto w-[150px] mt-[30px]">
      <button
        @click="moveRightToLeft"
        class="bg-purple-200 w-[150px] text-center h-[40px] bg-color-navy text-white"
      >
        左に移動
      </button>
    </div>
  </div>
</template>

<style scoped></style>
