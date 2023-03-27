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
                    <n-image width="250" src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg" />

                    <n-image width="250"
                        src="https://gw.alipayobjects.com/zos/antfincdn/aPkFc8Sj7n/method-draw-image.svg" />
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
            <n-button type="primary" @click="doSettle">
                我已支付
            </n-button>
            <n-button type="tertiary" style="margin-left: 50px;" @click="cancel">
                取消支付
            </n-button>
        </div>


    </div>
</template>

<script setup>
import { NImageGroup, NImage, NSpace, NNumberAnimation, NButton } from 'naive-ui';
import { ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const amount = ref(0.00)
const numberAnimationInstRef = ref(null)
let route = useRoute();
let router = useRouter();
onMounted(() => {
    console.log(route.params)
    if (route.params) {
        amount.value = Number(route.params.amount);
        numberAnimationInstRef.value?.play()
    }
})

function doSettle() {
    router.push({
        name: 'Main',
        params: {
            doSettle: 1,
        }
    })
}

function cancel() {
    router.push({
        name: 'Main',
    })
}

</script>

<style></style>