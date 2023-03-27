<template>
    <Carousel />
    <ShopForm />
</template>
  
<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Carousel from "../components/Carousel.vue";
import ShopForm from "../components/ShopForm.vue";
import { NGradientText } from "naive-ui";
</script>
  
<style scoped>
.logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
    filter: drop-shadow(0 0 2em #249b73);
}
</style>
  