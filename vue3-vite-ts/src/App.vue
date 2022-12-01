<script setup lang="ts">
import { ref } from "vue";
import leftList from "./components/LeftList.vue";
import rightList from "./components/RightList.vue";

const inputVal = ref("");
const handleClick = () => {
  if(inputVal.value === "")  {
    alert("何か入力してください");
    return
  }
  console.log(inputVal.value);
};
</script>

<template>
  <div class="flex w-[700px] m-auto justify-between mt-10">
    <!-- <leftList msg="leftList" />
    <rightList msg="rightList" /> -->
    <form type="text" >
      <input type="text" v-model="inputVal" />
      <button type="button" @click="handleClick()">test
      </button>
    </form>
  </div>
</template>

<style scoped></style>
