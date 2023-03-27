<template>
    <div class="m-auto mt-20 p-15 f-c-c rounded-10 card-shadow bg-white bg-opacity-60" dark:bg-dark>
        <!-- <div hidden md:block px-20 py-35>
            <img src="../assets/images/login_banner.webp" w-full alt="login_banner" />
        </div> -->

        <!-- <div flex-col px-20 py-35> -->

        <div>
            <h5 f-c-c m-auto text-8 text-center font-normal color="#6a6a6a">Welcome!</h5>
            <div mt-10>
                <n-input v-model:value="loginInfo.name" autofocus class="text-8 items-center pl-10" placeholder="请输入您的名字"
                    :maxlength="20" />
            </div>

            <div mt-10 style="text-align: center;">
                <n-button style="width: 60%;" rounded-5 type="primary" :loading="loading" @click="handleLogin">
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

const loginInfo = ref({
    name: ''
})
const router = useRouter();

getUserInfo();

async function getUserInfo() {
    const res = await invoke('get_user_info', { 'flag': 0 });
    if (res) {
        let obj = JSON.parse(res)
        if (obj && obj.name) {
            loginInfo.value.name = res;
            router.push({
                name: 'Main'
            })
        }
    }
}

async function handleLogin() {
    if (loginInfo.value.name) {
        const res = await invoke('write_user_info', { 'name': loginInfo.value.name });
        router.push({
            name: 'Main'
        })
    }
}
</script>

<style></style>