<template>
    <div class="m-auto mt-65 p-8 f-c-c rounded-10 bg-white bg-opacity-60" style="width:80%" dark:bg-dark>
        <div>
            <h3 f-c-c m-auto text-8 text-center font-normal color="#6a6a6a" style="font-family:方正舒体;font-size: 40px;">
                Welcome!</h3>
            <div mt-10 style="font-family:方正舒体;">
                <n-input v-model:value="loginInfo.name" autofocus class="text-8 items-center pl-10" placeholder="请输入您的名字"
                    :maxlength="20" />
            </div>

            <div mt-10 style="text-align: center;">
                <n-button style="width: 60%;font-family:方正舒体;font-size: 30px;" rounded-5 color="#fcf4df"
                    text-color="#397971" :loading="loading" @click="handleLogin">
                    登录
                </n-button>
            </div>
        </div>
    </div>
</template>

<script setup>
import { NButton, NInput, useMessage } from 'naive-ui';
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from "@tauri-apps/api/tauri";

const loginInfo = ref({
    name: ''
})
const router = useRouter();
const message = useMessage();

getUserInfo();

async function getUserInfo() {
    const res = await invoke('get_user_info', { 'flag': 0 });
    let obj = JSON.parse(res)
    if (obj.code === 0) {
        console.log(obj);
        if (obj.data.name) {
            loginInfo.value.name = obj.data.name;
            console.log(loginInfo.value.name);
            router.push({
                name: 'Main'
            });
        }
    } else {
        message.error(obj.msg);
    }
}

async function handleLogin() {
    if (loginInfo.value.name) {
        await invoke('write_user_info', { 'name': loginInfo.value.name });
        router.push({
            name: 'Main'
        })
    }
}
</script>

<style></style>