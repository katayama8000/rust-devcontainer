<script setup lang="ts">
import { key } from "@/store";
import { ref } from "vue";
import { useStore } from "vuex";
const store = useStore(key);
const text = ref("Home");

const handleGetInvoice = () => {
  console.log("Get Invoice");
  store.dispatch("GET_INVOICES");
};

function logout() {
  store.dispatch("logout");
}

const test = () => {
  store.dispatch("test");
};
</script>

<template>
  <h1>{{ text }}</h1>
  <button @click="handleGetInvoice">getInvoice</button>
  <button @click="logout">logout</button>
  <button @click="test">test</button>
</template>

<style lang="scss" scoped></style>
