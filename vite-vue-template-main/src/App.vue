<script setup lang="ts"></script>

<template>
  <nav>
    <ul>
      <li><router-link to="/">Home</router-link></li>
      <li><router-link to="/signup">Signup</router-link></li>
      <li><router-link to="/signin">Signin</router-link></li>
    </ul>
  </nav>
  <main>
    <RouterView class="contents" />
  </main>
</template>

<style lang="scss" scoped></style>
