<template>
    <div>
        <n-space vertical>
            <n-spin :show="show">
                <template #description>
                    {{ loadingtext }}
                </template>
                <n-space vertical>
                    <n-layout>
                        <n-layout-header :inverted="inverted" bordered>
                            <n-menu id="topMenu" mode="horizontal" :inverted="inverted" :options="menuOptions"
                                :on-update:value="changeMenu" />
                            <!-- <n-gradient-text v-if="arrearsAmount > 0">{{ arrearsAmount }}元</n-gradient-text> -->
                            <n-input v-model:value="searchText" id="searchBox"
                                style="min-width: 120px;float: right;margin-top: 5px;margin-right: 5px;" autosize autofocus
                                type="text" placeholder="搜索商品名称" />
                        </n-layout-header>
                        <n-layout has-sider>
                            <n-layout-sider bordered show-trigger collapse-mode="width" :collapsed-width="64" :width="140"
                                :native-scrollbar="false" :inverted="inverted" style="height: 320px">
                                <n-menu id="productMenu" :inverted="inverted" :collapsed-width="64"
                                    :collapsed-icon-size="22" :options="productOptions"
                                    :on-update:value="changeProductType" />
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
                                                        <n-button circle text style="margin-left: 15px;"
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
                                                        <n-button circle text style="margin-left: 10px;"
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
                                                    <n-button-group v-if="item.count > 0">
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
                                                    <n-button v-else-if="isRoot" @click="del(item.stock_sn)">
                                                        <template #icon>
                                                            <n-icon>
                                                                <mood-sad />
                                                            </n-icon>
                                                        </template>
                                                        删除商品
                                                    </n-button>
                                                    <n-button v-else @click="AddShopMessage(item.stock_sn)">
                                                        <template #icon>
                                                            <n-icon>
                                                                <mood-sad />
                                                            </n-icon>
                                                        </template>
                                                        快点进货
                                                    </n-button>
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
                    <n-card style="width: 600px" title="增加库存" :bordered="false" size="huge" role="dialog"
                        :aria-modal="true">
                        <template #header-extra>
                            {{ countAddName }}
                        </template>
                        <n-input-number v-model:value="countAddNum" :allow-input="true" clearable min="0" />
                        <template #footer>
                            <div style="text-align: center;width: 100%;">
                                <n-button @click="countAddConfirm">确认</n-button>
                            </div>
                        </template>
                    </n-card>
                </n-modal>
                <div style="text-align: right;width: 100%;margin-top: 5px;display: inline-block;">
                    <n-button color="#fcf4df" text-color="#397971" round @click="getProductList"
                        style="margin-right: 5px;font-family:方正舒体;font-size: large;" :strong="true">
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
                    <n-button id="settleButton" color="#487c78" text-color="#fff0e2"
                        style="font-family:方正舒体;font-size: 20px;" round @click="doSettle" :disabled="settleDisabled">
                        结算
                    </n-button>
                </div>
            </n-spin>
        </n-space>
    </div>
</template>

<script setup>

import { h, ref, computed, onMounted } from "vue";
import { useRouter, useRoute } from "vue-router";
import {
    NCard, NSpace, NBadge, NButtonGroup,
    NButton, NIcon, NLayout, NLayoutSider,
    NMenu, NLayoutFooter, NLayoutHeader,
    NGradientText, useMessage, NSpin, NImage,
    NRate, NInput, NPopover, NModal, NInputNumber
} from "naive-ui";
import { MdAdd, MdRemove, MdThumbsUp as thumbsUp, MdThumbsDown as thumbsDown } from "@vicons/ionicons4";
import { Coffee, Cup, BorderAll, Meat, History, MoodSad, Star, ListNumbers, CreditCard } from "@vicons/tabler";
import { invoke } from "@tauri-apps/api/tauri";
import {
    checkUpdate,
    installUpdate,
} from '@tauri-apps/api/updater';
import { ask } from '@tauri-apps/api/dialog';

// mac_address
const GZ = "A036BC65234F";
const BLACK_LIST = [GZ];

const message = useMessage();
const router = useRouter();
let route = useRoute();
// 已选商品数
const selectedShopNum = ref(0)
// 总金额
const totalAmount = ref(0)
// 商品列表
const productList = ref([])
// 当前分类的type
const productType = ref(0)

const isRoot = ref(false)

const loadingtext = ref("可能会有点慢，没钱升级服务器带宽 (ó﹏ò｡)...");

const nativeInfo = ref({})

const settleDisabled = ref(false)

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
        label: "收藏",
        key: "4",
        icon: renderIcon(Star)
    },
    // {
    //     label: "小俞咖啡",
    //     key: "3",
    //     icon: renderIcon(Coffee)
    // },
]);

const menuOptions = computed(() => [
    {
        label: "历史订单",
        key: "1",
        icon: renderIcon(History),
    },
    {
        label: "消费榜单",
        key: "2",
        icon: renderIcon(ListNumbers),
    },
    {
        label: "支付欠款：" + arrearsAmount.value + "元",
        key: "3",
        icon: renderIcon(CreditCard),
        show: Number(arrearsAmount.value) > 0
    },
])

// 库存增加相关
const countAddName = ref("")
const countAddNum = ref(0)
const countAddStockSn = ref(-1)
const showModal = ref(false)
const repeatMap = new Map();

// 商品列表的遮罩是否显示
const show = ref(false)
// 商品列表样式反转
const inverted = ref(false)
// 当前登录人员信息
const userInfo = ref({})
// 搜索框内容
const searchText = ref("")
// 未支付数据
const unpayData = ref([])

//------------------------页面初始流程-----------------------------

getUserInfo();

onMounted(async () => {
    let cancelPay = false;
    if (route.params.productList) {
        productList.value = JSON.parse(route.params.productList);
        totalAmount.value = Number(route.params.amount).toFixed(2) || (0).toFixed(2);
        selectedShopNum.value = Number(route.params.num) || 0;
        repeatMap.clear();
        cancelPay = true;
    } else {
        await getProductList();
    }
    invoke('get_arrears_amount', {}).then(e => {
        let res = JSON.parse(e);
        if (res.code === 0) {
            unpayData.value = res.data;
        } else {
            console.error("获取欠款失败 => " + res.msg);
        }
    }).catch(e => {
        console.error("获取欠款失败 => " + e);
    })

    const live2d_path = "http://172.16.140.83:4002/assets/live2d/";

    Promise.all([
        loadExternalResource(live2d_path + "waifu.css", "css"),
        loadExternalResource(live2d_path + "live2d.min.js", "js"),
        loadExternalResource(live2d_path + "waifu-tips.js", "js")
    ]).then(() => {
        initWidget({
            waifuPath: live2d_path + "waifu-tips.json",
            // cdnPath: "https://fastly.jsdelivr.net/gh/fghrsh/live2d_api/",
            cdnPath: "http://172.16.140.83:4002/assets/live2d_api/",
            tools: ["hitokoto", "asteroids", "switch-model", "switch-texture", "photo", "info", "quit"]
        });
        if (cancelPay) {
            setTimeout(() => {
                showMessage("是有什么商品选错了吗，已为你保留订单信息，在支付界面点击上方商品标签，也可以取消部分商品哦~", 8000, 13)
            }, 500);
        }
    });

})
// 获取本地信息
getNativeInfo();
// 自动更新
lookingForUpdate();

function loadExternalResource(url, type) {
    return new Promise((resolve, reject) => {
        let tag;

        if (type === "css") {
            tag = document.createElement("link");
            tag.rel = "stylesheet";
            tag.href = url;
        }
        else if (type === "js") {
            tag = document.createElement("script");
            tag.src = url;
        }
        if (tag) {
            tag.onload = () => resolve(url);
            tag.onerror = () => reject(url);
            document.head.appendChild(tag);
        }
    });
}

let messageTimer;
function showMessage(text, timeout, priority) {
    if (!text || (sessionStorage.getItem("waifu-text") && sessionStorage.getItem("waifu-text") > priority)) return;
    if (messageTimer) {
        clearTimeout(messageTimer);
        messageTimer = null;
    }
    sessionStorage.setItem("waifu-text", priority);
    const tips = document.getElementById("waifu-tips");
    tips.innerHTML = text;
    tips.classList.add("waifu-tips-active");
    messageTimer = setTimeout(() => {
        sessionStorage.removeItem("waifu-text");
        tips.classList.remove("waifu-tips-active");
    }, timeout);
}


//------------------------Rust 函数-------------------------------

async function getProductList() {
    show.value = true;
    let data = await invoke("get_product_list", {});
    let res = JSON.parse(data)
    if (res.code === 0) {
        productList.value = res.data;
    } else {
        console.log(res)
        message.error("获取商品数据失败" + res.msg);
    }
    selectedShopNum.value = 0;
    totalAmount.value = 0;
    show.value = false;
    repeatMap.clear();
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

async function del(stock_sn) {
    invoke('delete_product', { 'stockSn': stock_sn }).then(e => {
        message.success("删除成功");
        getProductList();
    }).catch(e => {
        message.error("删除失败！" + e);
    })
}

function AddShopMessage(stock_sn) {

    let times = repeatMap.get(stock_sn) || 0;
    if (Number(times) < 5) {
        message.success('已收到您的催货请求，请耐心等候！');
    } else {
        // 也就鸽子能触发了
        message.success('差不多就行了，再催也没用！');
    }
    repeatMap.set(stock_sn, times + 1);
}

let filterProductList = computed(() => {
    if (searchText.value !== "") {
        return productList.value.filter(e => ~e.product_name.indexOf(searchText.value))
    } else {
        return productList.value.filter(e => productType.value === 0 || (e.product_type & productType.value) != 0)
    }
});

// 欠款金额
let arrearsAmount = computed(() => {
    if (unpayData.value.length === 0) return 0;
    let total = 0;
    unpayData.value.forEach(e => {
        total += e.amount;
    })
    return total.toFixed(2);
})

function changeProductType(key) {
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
    await invoke('add_comment', { 'state': 1, 'stockSn': stock_sn }).catch(e => {
        console.log(e)
    })
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
    invoke('add_comment', { 'state': 2, 'stockSn': stock_sn }).catch(e => {
        console.log(e)
    })
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
        showMessage("您还什么都没选购呢？在想要购买的商品卡片下 点击加号按钮 即可加入购物车~", 8000, 13);
        return;
    }
    let totalAmount0 = Number(totalAmount.value) || 0;
    // if (~BLACK_LIST.indexOf(nativeInfo.value.macAddress)) {
    //     totalAmount0 = totalAmount0 * 1.5;
    // }

    let temp = productList.value;
    router.push({
        name: "Qrcode",
        params: {
            productList: JSON.stringify(temp),
            amount: totalAmount0,
            num: selectedShopNum.value
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

function changeMenu(key) {
    if (key == 1) {
        router.push({
            name: "History",
            params: {
                "name": userInfo.value.name
            }
        })
    } else if (key == 2) {
        router.push({
            name: "ShoppingList",
        })
    } else if (key == 3) {
        router.push({
            name: "Qrcode",
            params: {
                productList: JSON.stringify(unpayData.value),
                amount: Number(arrearsAmount.value),
                num: -1
            }
        })
    }
}

// 添加库存
function countAdd(item) {
    if (isRoot.value === true) {
        showModal.value = true
        countAddName.value = item.product_name
        countAddStockSn.value = item.stock_sn
        countAddNum.value = 0
    }
}

function countAddConfirm() {
    if (!~countAddStockSn.value || countAddNum.value <= 0) return;

    invoke('add_product_count', { 'stockSn': countAddStockSn.value, 'num': countAddNum.value, 'name': countAddName.value })
        .then(e => {
            if (e.code == 1) {
                console.log(e)
                message.error("库存增加失败" + e)
            } else {
                message.success("添加成功")
                getProductList();
            }
        })
        .catch(e => {
            console.log(e)
            message.error("库存增加失败" + e)
        })
    countAddStockSn.value = -1
    countAddName.value = ""
    showModal.value = false
}

async function lookingForUpdate() {
    try {
        const { shouldUpdate, manifest } = await checkUpdate()
        if (!shouldUpdate) return;
        const res = await ask(manifest.body, { title: '更新提示', type: 'info' });
        loadingtext.value = "正在升级中，请耐心等候，更新完成后程序会自动重启！"
        show.value = true;
        if (res) {
            await installUpdate()
        }
        show.value = false;
        loadingtext.value = "可能会有点慢，没钱升级服务器带宽(ó﹏ò｡)..."
    } catch (e) {
        show.value = false;
        console.error(e);
        message.error("出现未知异常，自动更新失败，请联系俞晨星！");
    }

}


async function getNativeInfo() {
    await invoke('get_native_info', {}).then(e => {
        let res = JSON.parse(e);
        if (res.code === 0) {
            nativeInfo.value = res.data;
        } else {
            if (res.msg == "wrong user") {
                for (let i = 0; i < 10; i++) {
                    message.warning("请不要随意更改登录人员的名称！！！结算功能已受限！！！");
                }
                settleDisabled.value = true;
            }
        }

    }).catch(e => { console.error("get_native_info error => " + e) })
}



</script>


<style scoped>
.n-card {
    max-width: 300px;
}
</style>