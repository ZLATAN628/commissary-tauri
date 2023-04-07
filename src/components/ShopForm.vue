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
                            <n-menu mode="horizontal" :inverted="inverted" :options="menuOptions"
                                :on-update:value="changeMenu" />
                            <n-input v-model:value="searchText"
                                style="min-width: 150px;float: right;margin-top: 5px;margin-right: 5px;" autosize autofocus
                                type="text" placeholder="搜索商品名称" />
                        </n-layout-header>
                        <n-layout has-sider>
                            <n-layout-sider bordered show-trigger collapse-mode="width" :collapsed-width="64" :width="140"
                                :native-scrollbar="false" :inverted="inverted" style="height: 320px">
                                <n-menu :inverted="inverted" :collapsed-width="64" :collapsed-icon-size="22"
                                    :options="productOptions" :on-update:value="changeProductType" />
                            </n-layout-sider>
                            <n-layout style="height: 320px;background-color: #abc8ce;" :native-scrollbar="false">
                                <div style="display: inline-block;font-size: 10px;">
                                    <div style="margin-left: 10px;margin-top: 10px; float: left;max-width: 200px;max-height: 350px;display: inline-block;"
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
                                            <template #footer>
                                                <n-rate size="small" allow-half readonly :value="item.rate" />
                                                <n-popover trigger="hover">
                                                    <template #trigger>
                                                        <n-button tertiary circle text style="margin-left: 15px;"
                                                            @click="goodComment(item.stock_sn)">
                                                            <template #icon>
                                                                <n-icon color="#397971">
                                                                    <thumbs-up />
                                                                </n-icon>
                                                            </template>
                                                        </n-button>
                                                    </template>
                                                    <span>如果觉得好吃的话 就点个赞吧 ♪(^∇^*) </span>
                                                </n-popover>
                                                <n-popover trigger="hover">
                                                    <template #trigger>
                                                        <n-button tertiary circle text style="margin-left: 10px;"
                                                            @click="badComment(item.stock_sn)">
                                                            <template #icon>
                                                                <n-icon color="#abc8ce">
                                                                    <thumbs-down />
                                                                </n-icon>
                                                            </template>
                                                        </n-button>
                                                    </template>
                                                    <span>看起来是很难吃的东西了 o(一︿一+)o</span>
                                                </n-popover>

                                            </template>
                                            <template #action>
                                                <n-space :size="24" item-style="text-align: right;">
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
                                                    <n-badge :max="item.count" :processing="true" color="green">
                                                        <template #default>
                                                            <label style="font-family:方正舒体;" @click="countAdd(item)">
                                                                <label style="font-size: large;">{{ item.cur }}</label>/
                                                                {{ item.count }}
                                                            </label>
                                                        </template>
                                                    </n-badge>
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
                <n-modal v-model:show="showModal">
                    <n-card style="width: 600px" title="增加库存" :bordered="false" size="huge" role="dialog" aria-modal="true">
                        <template #header-extra>
                            {{ countAddName }}
                        </template>
                        <n-input></n-input>
                        <template #footer style="text-align: center;">
                            <n-button>确认</n-button>
                        </template>
                    </n-card>
                </n-modal>
                <div style="text-align: right;width: 100%;margin-top: 5px;display: inline-block;">
                    <n-button color="#fcf4df" text-color="#397971" round @click="getProductList"
                        style="margin-right: 5px;font-family:方正舒体;font-size: large;" strong="true">
                        刷新库存
                    </n-button>
                    <n-button color="#fcf4df" text-color="#397971" strong round @click="insertProduct"
                        style="margin-right: 5px;font-family:方正舒体;font-size: large;" v-show="isRoot">
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
    NGradientText, useMessage, NSpin, NImage,
    NRate, NInput, NPopover, NModal
} from "naive-ui";
import { MdAdd, MdRemove, MdThumbsUp as thumbsUp, MdThumbsDown as thumbsDown } from "@vicons/ionicons4";
import { Coffee, Cup, BorderAll, Meat, History } from "@vicons/tabler";
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

const isRoot = ref(false)

const productOptions = ref([
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

const menuOptions = ref([
    {
        label: "商品分类",
        key: "0",
        disabled: true,
        icon: renderIcon(BorderAll)
    },
    {
        label: "历史订单",
        key: "1",
        icon: renderIcon(History),
    },
])

const countAddName = ref("")

const showModal = ref(false)
// 商品列表的遮罩是否显示
const show = ref(false)
// 商品列表样式反转
const inverted = ref(false)
// 当前登录人员信息
const userInfo = ref({})
// 搜索框内容
const searchText = ref("")

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
    if (searchText.value !== "") {
        return productList.value.filter(e => ~e.product_name.indexOf(searchText.value))
    } else {
        return productList.value.filter(e => productType.value === 0 || e.product_type == productType.value)
    }

});

function changeProductType(key, item) {
    productType.value = Number(key);
}

async function goodComment(stock_sn) {
    productList.value.filter(e => e.stock_sn === stock_sn).forEach(e => {
        if (e.state === 1) {
            // do nothing
        } else if (e.state === 2) {
            e.rate = Number((e.good + 1) / (e.good + e.bad) * 5).toFixed(2)
            e.state = 1
            e.good += 1
            e.bad -= 1
        } else {
            e.rate = Number((e.good + 1) / (e.good + e.bad + 1) * 5).toFixed(2)
            e.state = 1
            e.good += 1
        }
    });
    await invoke('add_comment', { 'state': 1, 'stockSn': stock_sn })
}

function badComment(stock_sn) {
    productList.value.filter(e => e.stock_sn === stock_sn).forEach(e => {
        if (e.state === 1) {
            e.rate = Number((e.good - 1) / (e.good + e.bad) * 5).toFixed(2)
            e.state = 2
            e.good -= 1
            e.bad += 1
        } else if (e.state === 2) {
            // do nothing
        } else {
            e.rate = Number((e.good - 1) / (e.good + e.bad + 1) * 5).toFixed(2)
            e.state = 2
            e.bad += 1
        }
    });

    // 很tm离谱，前端传参一定要驼峰形式，后端接收要下划线形式，不然报错，妈的 stockSn -> stock_sn
    invoke('add_comment', { 'state': 2, 'stockSn': stock_sn })
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
            isRoot.value = true;
        }
        userInfo.value = { "name": res.data.name }
    }
}

function changeMenu() {
    router.push({
        name: "History",
        params: {
            "name": userInfo.value.name
        }
    })
}

// 添加库存
async function countAdd(item) {
    showModal.value = true
    countAddName.value = item.product_name
}


</script>


<style scoped>
.n-card {
    max-width: 300px;
}
</style>