<template>
    <Carousel />
    <div class="m-auto p-15 f-c-c rounded-10 card-shadow bg-white bg-opacity-60" dark:bg-dark>
        <!-- <div hidden md:block px-20 py-35>
            <img src="../assets/images/login_banner.webp" w-full alt="login_banner" />
        </div> -->

        <!-- <div flex-col px-20 py-35> -->

        <div>
            <!-- <h5 f-c-c text-24 font-normal color="#6a6a6a"><icon-custom-logo mr-10 text-50 color-primary />{{ title }}</h5> -->
            <div mt-20>
                <n-input v-model:value="loginInfo.name" autofocus class="text-8 items-center pl-10" placeholder="登录用户"
                    :maxlength="20" />
            </div>

            <div mt-10>
                <n-button w-full rounded-5 type="primary" :loading="loading" @click="handleLogin">
                    登录
                </n-button>
            </div>
        </div>
    </div>
</template>

<script setup>
import { NImageGroup, NImage, NSpace, NNumberAnimation, NButton, NInput, NCheckbox } from 'naive-ui';
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from "@tauri-apps/api/tauri";
import Carousel from "../components/Carousel.vue";

const loginInfo = ref({
    name: ''
})
const router = useRouter();

getUserInfo();

async function getUserInfo() {
    const res = await invoke('get_user_info', {});
    console.log(res);
    if (res) {
        let obj = JSON.parse(res)
        if (obj && obj.name) {
            loginInfo.value.name = res;
            router.push({
                name: 'Main',
                params: {
                    oper: loginInfo.value.name
                }
            })
        }
    }
}

async function handleLogin() {
    if (loginInfo.value.name) {
        const res = await invoke('write_user_info', { 'name': loginInfo.value.name });
        router.push({
            name: 'Main',
            params: {
                oper: loginInfo.value.name
            }
        })
    }
}
</script>

<style></style>