<template>
    <div>
        <n-space vertical>
            <n-spin :show="show">
                <template #description>
                    可能会有点慢，没钱升级服务器带宽 (ó﹏ò｡)...
                </template>
                <n-space vertical>
                    <n-layout>
                        <n-layout-header :inverted="inverted" bordered>
                            <n-menu mode="horizontal" :inverted="inverted" :options="menuOptions" />
                        </n-layout-header>
                        <n-layout has-sider>
                            <n-layout-sider bordered show-trigger collapse-mode="width" :collapsed-width="64" :width="140"
                                :native-scrollbar="false" :inverted="inverted" style="height: 320px">
                                <n-menu :inverted="inverted" :collapsed-width="64" :collapsed-icon-size="22"
                                    :options="menuOptions" :on-update:value="changeProductType" />
                            </n-layout-sider>
                            <n-layout style="height: 320px;background-color: #abc8ce;" :native-scrollbar="false">
                                <div style="display: inline-block;font-size: 10px;">
                                    <div style="margin-left: 10px;margin-top: 10px; float: left;max-width: 200px;max-height: 300px;display: inline-block;"
                                        v-for="item in filterProductList" key="item.stock_sn">
                                        <n-card size="small" tag="span" hoverable>
                                            <template #cover style="width 80px;height: 80px;">
                                                <n-image :src="item.image" width="198" height="193" />
                                            </template>
                                            <template #header>
                                                <label style="font-size: 10px;">{{ item.product_name }}</label>
                                            </template>
                                            <template #header-extra>
                                                <label style="font-size: 10px;">单价： {{ item.price }} 元</label>
                                            </template>
                                            <template #action>
                                                <n-space :size="24" item-style="text-align: right">
                                                    <n-badge :value="item.cur" :max="item.count" />
                                                    <n-button-group>
                                                        <n-button @click="Add(item.stock_sn)">
                                                            <template #icon>
                                                                <n-icon><md-add /></n-icon>
                                                            </template>
                                                        </n-button>
                                                        <n-button @click="Sub(item.stock_sn)">
                                                            <template #icon>
                                                                <n-icon><md-remove /></n-icon>
                                                            </template>
                                                        </n-button>
                                                    </n-button-group>
                                                </n-space>
                                            </template>
                                        </n-card>
                                    </div>
                                </div>
                            </n-layout>
                        </n-layout>
                        <n-layout-footer :inverted="inverted" bordered>
                            良心价格 童叟无欺 走过路过 不要错过
                        </n-layout-footer>
                    </n-layout>
                </n-space>
                <div style="text-align: right;width: 100%;margin-top: 5px;display: inline-block;">
                    <n-button color="#fcf4df" text-color="#397971" round @click="getProductList"
                        style="margin-right: 5px;font-family:方正舒体;font-size: large;" strong="true">
                        刷新库存
                    </n-button>
                    <n-button color="#fcf4df" text-color="#397971" strong round @click="insertProduct"
                        style="margin-right: 5px;font-family:方正舒体;font-size: large;" v-show="buttonShow">
                        发布商品
                    </n-button>
                    <n-gradient-text font-mono font-extrabold type="primary" style="margin-right: 5px;font-size: 20px;">
                        <label style="font-family:方正舒体;">
                            已选
                            <label style="color: #fff0e2;font-size: larger;">{{ selectedShopNum }}</label>
                            件商品，共计
                            <label style="color: #fff0e2;font-size: larger;">{{ totalAmount }}</label>
                            元
                        </label>
                    </n-gradient-text>
                    <n-button color="#487c78" text-color="#fff0e2" style="font-family:方正舒体;font-size: 20px;" round
                        @click="doSettle">
                        结算
                    </n-button>
                </div>
            </n-spin>
        </n-space>
    </div>
</template>

<script setup>
import { h, ref, computed } from "vue";
import { useRouter } from "vue-router";
import {
    NCard, NSpace, NBadge, NButtonGroup,
    NButton, NIcon, NLayout, NLayoutSider,
    NMenu, NLayoutFooter, NLayoutHeader,
    NGradientText, useMessage, NSpin, NImage
} from "naive-ui";
import { MdAdd, MdRemove } from "@vicons/ionicons4";
import { Coffee, Cup, BorderAll, Meat } from "@vicons/tabler";
import { invoke } from "@tauri-apps/api/tauri";

const message = useMessage();
const router = useRouter();
// 已选商品数
const selectedShopNum = ref(0)
// 总金额
const totalAmount = ref(0)
// 商品列表
const productList = ref([])
// 当前分类的type
const productType = ref(0)

const buttonShow = ref(false)

const menuOptions = ref([
    {
        label: "全部",
        key: "0",
        icon: renderIcon(BorderAll)
    },
    {
        label: "干货",
        key: "1",
        icon: renderIcon(Meat),
    },
    {
        label: "饮料",
        key: "2",
        icon: renderIcon(Cup)
    },
    {
        label: "小俞咖啡",
        key: "3",
        icon: renderIcon(Coffee)
    },
]);

const show = ref(false)

const inverted = ref(false)

//------------------------页面初始流程-----------------------------

getProductList();
getUserInfo();

//------------------------Rust 函数-------------------------------

async function getProductList() {
    show.value = true;
    let data = await invoke("get_product_list", {});
    let res = JSON.parse(data)
    if (res.code === 0) {
        productList.value = res.data;
    } else {
        message.error("获取商品数据失败" + res.msg);
    }
    selectedShopNum.value = 0;
    totalAmount.value = 0;
    show.value = false;
}



//------------------------Js 函数---------------------------------
function renderIcon(icon) {
    return () => h(NIcon, null, { default: () => h(icon) });
}

function Add(stock_sn) {
    productList.value.filter(e => e.stock_sn === stock_sn).forEach(e => {
        if (e.cur < e.count) {
            e.cur += 1;
            selectedShopNum.value += 1;
            totalAmount.value = Number(Number(totalAmount.value) + e.price).toFixed(2) || 0;
        } else {
            message.warning('点不动就是卖完啦!（´ސު`）')
        }
    });
}

function Sub(stock_sn) {
    productList.value.filter(e => e.stock_sn === stock_sn).forEach(e => {
        if (e.cur > 0) {
            e.cur -= 1;
            selectedShopNum.value -= 1;
            totalAmount.value = Number(Number(totalAmount.value) - e.price).toFixed(2) || 0;
            console.log(totalAmount.value)
        }
    });
}

let filterProductList = computed(() => {
    return productList.value.filter(e => productType.value === 0 || e.product_type == productType.value)
});

function changeProductType(key, item) {
    productType.value = Number(key);
}

/**
 * 发布商品
 */
function insertProduct() {
    router.push({
        name: "Product",
    });
}

async function doSettle() {
    let settle_list = productList.value.filter(e => e.cur > 0);
    if (!settle_list || settle_list.length <= 0) {
        return;
    }

    let temp = productList.value;
    router.push({
        name: "Qrcode",
        params: {
            productList: JSON.stringify(temp),
            amount: Number(totalAmount.value)
        }
    })
}


async function getUserInfo() {
    let data = await invoke('get_user_info', { 'flag': 1 });
    let res = JSON.parse(data);
    if (res.code === 0) {
        if (res.data.name === '俞晨星' || res.data.name === '张建华') {
            buttonShow.value = true;
        }
    }
}


</script>


<style scoped>
.n-card {
    max-width: 300px;
}
</style>