<template>
    <div style="height: 550px;">
        <div style="text-align: center;">
            <p style="color: red;font-size: xx-large;">
                小本经营 诚信买卖
            </p>
        </div>
        <div style="margin-left: 50px;margin-top: 40px">
            <n-image-group>
                <n-space>
                    <n-image width="250" height="350" src='/src/resource/alipay.jpg' />
                    <n-image width="250" height="350" src="/src/resource/wechatpay.jpg" />
                </n-space>
            </n-image-group>
        </div>
        <div style="text-align: center;">
            <p style="color: red;font-size: xx-large;">
                您本次购物结算金额为
                <n-number-animation ref="numberAnimationInstRef" :from="0.00" :to="amount" :active="true" :precision="2" />
                元
            </p>
        </div>
        <div style="text-align: center;">
            <n-button type="primary" @click="doSettle" :disabled="paybutton">
                我已支付
            </n-button>
            <n-button type="tertiary" style="margin-left: 50px;" @click="cancel">
                取消支付
            </n-button>
        </div>
    </div>
</template>

<script setup>
import { NImageGroup, NImage, NSpace, NNumberAnimation, NButton, useMessage } from 'naive-ui';
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from "@tauri-apps/api/tauri";

const message = useMessage();
const amount = ref(0.00)
const numberAnimationInstRef = ref(null)
const paybutton = ref(false)
let route = useRoute();
let router = useRouter();
let productList;
onMounted(() => {
    if (route.params.productList) {
        productList = JSON.parse(route.params.productList);
        amount.value = Number(route.params.amount);
        numberAnimationInstRef.value?.play()
    }
})

async function doSettle() {
    paybutton.value = true;
    productList.forEach(element => {
        element.image = "";
    });
    let data = JSON.stringify(productList);
    let res = await invoke('do_settle', { "data": data })
    if (res === '结算成功') {
        message.success("结算成功！！！ 请自行前往张建华身后的零食柜领取商品！！！")
    } else {
        message.error("结算失败！！！请联系俞晨星！！！")
    }
    router.push({
        name: 'Main',
    })
}

function cancel() {
    router.push({
        name: 'Main',
    })
}

</script>

<style></style>