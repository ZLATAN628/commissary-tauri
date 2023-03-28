<template>
    <div style="height: 240px;">
        <n-carousel show-arrow autoplay>
            <img v-for="item in getCarouselList" class="carousel-img" :src="item.image">
        </n-carousel>
    </div>
</template>

<script setup>
import { ref, computed } from "vue";
import { NCarousel } from "naive-ui";
import { invoke } from "@tauri-apps/api/tauri";

const carouselList = ref([]);

invoke('get_carousel_list').then(data => {
    let res = JSON.parse(data);
    if (res.code === 0) {
        carouselList.value = res.data;
    }
}).catch(e => {
    console.log(e);
});

let getCarouselList = computed(() => {
    return carouselList.value
});


</script>

<style scoped>
.carousel-img {
    width: 100%;
    height: 240px;
    object-fit: cover;
}
</style>