<script setup lang="ts">
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
  <div>
    <div v-if="leftItems.length > 0" class="w-[300px] border-navy text-center">
      <div v-for="item in leftItems" :key="item">
        <div class="p-1">
          {{ item }}
        </div>
      </div>
    </div>
    <div class="m-auto w-[150px] mt-[30px]">
      <button
        @click="moveLeftToRight"
        class="bg-purple-200 w-[150px] text-center h-[40px]"
      >
        右に移動
      </button>
    </div>
  </div>
</template>

<style scoped></style>
