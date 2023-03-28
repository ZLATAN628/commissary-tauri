<template>
  <n-message-provider>
    <div class="ct">
      <div style="width: 100%;height: 110px;">
        <!-- <n-gradient-text gradient="linear-gradient(90deg, red 0%, green 50%, blue 100%)">
          <h1>云经济小卖部</h1>
        </n-gradient-text> -->
      </div>
      <router-view />
    </div>
  </n-message-provider>
</template>

<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import Carousel from "./components/Carousel.vue";
// import ShopForm from "./components/ShopForm.vue";
import { NGradientText, NMessageProvider } from "naive-ui";
</script>

<style scoped>
/* .logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
} */
.ct {
  background-image: url('./resource/background.png');
  position: fixed;
  top: 0;
  left: 0;
  width: 590px;
  height: 790px;
  background-size: 590px 790px;
}
</style>
