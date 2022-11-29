<script setup lang="ts">
import { reactive, ref } from "vue";
import { useRoute } from "vue-router";
import { useStore } from "vuex";
import { key } from "../store";
const route = useRoute();
const store = useStore(key);

type formState = {
  password: string;
  email: string;
  isShowPassword: boolean;
};

const formState = reactive<formState>({
  email: "",
  password: "",
  isShowPassword: false,
});

const handleSignUp = () => {
  console.log("handleSignUp");
};
</script>

<template>
  <h1>signup</h1>
  <div>
    <input type="text" v-model="formState.email" />
    <input
      v-model="formState.password"
      :type="formState.isShowPassword ? 'text' : 'password'"
    />
    <button @click="formState.isShowPassword = !formState.isShowPassword">
      isShow
    </button>
    <button @click="handleSignUp()">{{ route.name }}</button>
  </div>
</template>

<style lang="scss" scoped></style>
