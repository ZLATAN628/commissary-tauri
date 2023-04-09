<template>
    <div style="height: 550px;">
        <div style="margin-left: 130px;">
            <p style="color:aquamarine;font-size: 35px;font-family:方正舒体;">
                小本经营 诚信买卖
            </p>
        </div>


        <div style="margin-left: 50px;margin-top: 152px">
            <n-image-group>
                <n-space>
                    <n-image width="250" height="310" src="http://172.16.140.83:9000/commissary-tauri/code/alipay.jpg" />
                    <n-image width="250" height="310" src="http://172.16.140.83:9000/commissary-tauri/code/wechatpay.jpg" />
                </n-space>
            </n-image-group>
        </div>
        <div style="margin-top: 10px;">
            <label style="color: red;margin-left: 40px;font-size: 16px;"> 支付成功后请点击 => </label>
            <n-button type="primary" @click="doSettle" :disabled="paybutton" size="large">
                我已支付
            </n-button>
            <n-button type="error" style="margin-left: 50px;" @click="cancel" size="large">
                取消支付
            </n-button>
        </div>
        <div style="text-align: center;margin-top: 10px;font-family:方正舒体;">
            <n-gradient-text font-mono font-extrabold type="error" style="font-size: xx-large;">
                您本次购物结算金额为
                <label style="color: #fff0e2">
                    <n-number-animation ref="numberAnimationInstRef" :from="0.00" :to="amount" :active="true"
                        :precision="2" />
                </label>
                元
            </n-gradient-text>

        </div>
    </div>
</template>

<script setup>
import { NImageGroup, NImage, NSpace, NNumberAnimation, NButton, useMessage, NGradientText, NModal, NCard } from 'naive-ui';
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

let unlisten;
const message = useMessage();
const amount = ref(0.00)
const numberAnimationInstRef = ref(null)
const paybutton = ref(false)
let route = useRoute();
let router = useRouter();
let productList;
onMounted(async () => {
    if (route.params.productList) {
        productList = JSON.parse(route.params.productList);
        amount.value = Number(route.params.amount);
        numberAnimationInstRef.value?.play()
    }
    unlisten = await listen("tauri://close-requested", async (event) => {
        console.log("监听", event);
    })

})

async function doSettle() {
    paybutton.value = true;
    productList.forEach(element => {
        element.image = "";
    });
    let data = JSON.stringify(productList);
    let e = await invoke('do_settle', { "data": data })
    let res = JSON.parse(e);
    if (res.code === 0) {
        message.success("结算成功！！！ 请自行前往张建华身后的零食柜领取商品！！！")
    } else {
        console.log(res)
        message.error("结算失败！！！请联系俞晨星！！！" + res.msg)
    }
    router.push({
        name: 'Main',
    })
}

function cancel() {
    unlisten()
    router.push({
        name: 'Main',
    })
}

</script>
<style></style>