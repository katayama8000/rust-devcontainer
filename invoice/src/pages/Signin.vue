<script setup lang="ts">
import router from "@/router";
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

const handleSignIn = () => {
  console.log("handleSignUp");
  if (route.name === "signin") {
    store.dispatch(route.name, {
      email: formState.email,
      password: formState.password,
    });
  }
};
</script>

<template>
  <h1>signIn</h1>
  <div>
    <input type="text" placeholder="email" v-model="formState.email" />

    <input
      placeholder="password"
      v-model="formState.password"
      :type="formState.isShowPassword ? 'text' : 'password'"
    />
    <button @click="formState.isShowPassword = !formState.isShowPassword">
      isShow
    </button>
    <button @click="handleSignIn()">{{ route.name }}</button>
  </div>
</template>

<style lang="scss" scoped></style>
