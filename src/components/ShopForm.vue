<template>
    <div>
        <n-space vertical>
            <n-layout>
                <n-layout-header :inverted="inverted" bordered>
                    <n-menu mode="horizontal" :inverted="inverted" :options="menuOptions" />
                </n-layout-header>
                <n-layout has-sider>
                    <n-layout-sider bordered show-trigger collapse-mode="width" :collapsed-width="64" :width="140"
                        :native-scrollbar="false" :inverted="inverted" style="height: 320px">
                        <n-menu :inverted="inverted" :collapsed-width="64" :collapsed-icon-size="22" :options="menuOptions"
                            :on-update:value="changeProductType" />
                    </n-layout-sider>
                    <n-layout style="height: 320px;background-color: bisque;" :native-scrollbar="false">
                        <div style="display: inline-block;font-size: 10px;">
                            <div style="margin-left: 10px;margin-top: 10px; float: left;max-width: 200px;max-height: 300px;display: inline-block;background-color: black;"
                                v-for="item in filterProductList" key="item.stock_sn">
                                <n-card size="small" tag="span" hoverable>
                                    <template #cover>
                                        <img :src="item.image">
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
        <div style="text-align: right;margin-top: 10px;">
            <n-button type="info" strong secondary round @click="insertProduct" style="margin-right: 100px;">
                发布商品
            </n-button>
            <n-gradient-text type="warning" style="margin-right: 40px;font-size: 20px;">
                已选
                {{ selectedShopNum }}
                件商品，共计
                {{ totalAmount }}
                元
            </n-gradient-text>
            <n-button type="warning" strong secondary round @click="doSettle">
                结算
            </n-button>
        </div>


    </div>
</template>

<script setup>
import { h, ref, watch, computed, onMounted } from "vue";
import { useRouter, useRoute } from "vue-router";
import {
    NCard, NSpace, NBadge, NButtonGroup,
    NButton, NIcon, NLayout, NLayoutSider,
    NMenu, NLayoutFooter, NLayoutHeader,
    NGradientText, NNumberAnimation
} from "naive-ui";
import { MdAdd, MdRemove } from "@vicons/ionicons4";
import {
    BookOutline as BookIcon,
    PersonOutline as PersonIcon,
    WineOutline as WineIcon
} from "@vicons/ionicons5";
import { invoke } from "@tauri-apps/api/tauri";

const router = useRouter();
// 已选商品数
const selectedShopNum = ref(0)
// 总金额
const totalAmount = ref(0)
// 商品列表
const productList = ref([])
// 当前分类的type
const productType = ref(0)

const menuOptions = ref([
    {
        label: "全部",
        key: "0",
        icon: renderIcon(BookIcon)
    },
    {
        label: "干货",
        key: "1",
        icon: renderIcon(BookIcon),
    },
    {
        label: "饮料",
        key: "2",
        icon: renderIcon(BookIcon)
    },
    {
        label: "小俞咖啡",
        key: "3",
        icon: renderIcon(BookIcon)
    },
]);

const showModal = ref(false)

const inverted = ref(true)

const route = useRoute();

//------------------------页面初始流程-----------------------------

getProductList();

//------------------------Rust 函数-------------------------------

async function getProductList() {
    let data = await invoke("get_product_list", {});
    productList.value = JSON.parse(data);
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
            showModal.value = true;
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

function getSrc(index) {
    index = index % 3 + 1;
    return "/src/resource/" + index + ".jpeg"
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

async function doSettle(type = 0) {
    let settle_list = productList.value.filter(e => e.cur > 0);
    if (!settle_list || settle_list.length <= 0) {
        return;
    }
    if (type === 1) {
        let res = await invoke('do_settle', { "data": JSON.stringify(settle_list) })
    } else {
        router.push({
            name: "Qrcode",
            params: {
                amount: Number(totalAmount.value)
            }
        })

    }


}

onMounted(() => {
    if (route.params && route.params.doSettle == 1 && productList.value.length > 0) {
        // 支付成功 本地结算
        doSettle(1)
    }
})

</script>


<style scoped>
.n-card {
    max-width: 300px;
}
</style>