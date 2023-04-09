<template>
    <Carousel />

    <n-space vertical>
        <n-data-table :columns="columns" :data="payRecordList" min-height="322px" flex-height striped
            :single-line="false" />
    </n-space>



    <div style="text-align: center;width: 100%;margin-top: 5px;display: inline-block;">
        <n-button color="#fcf4df" text-color="#397971" round @click="goBack"
            style="font-family:方正舒体;font-size: larger;width: 40%;margin-top: 10px;" strong="true">
            返回
        </n-button>
    </div>
</template>

<script setup>
import { NSpace, NButton, NDataTable, useMessage } from 'naive-ui';
import { ref, onMounted, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import Carousel from "../components/Carousel.vue";
import { invoke } from "@tauri-apps/api/tauri";

const username = ref("");
let route = useRoute();
let router = useRouter();
let message = useMessage();

const payRecordList = ref([]);
const columns = ref([
    {
        title: "No",
        key: "no",
        width: 80,
        sorter: (row1, row2) => row1.no - row2.no
    },
    {
        title: "购买时间",
        key: "pay_time",
        width: 160,
        sorter: (row1, row2) => Date.parse(row1.pay_time) - Date.parse(row2.pay_time)
    },
    {
        title: "商品详情",
        key: "info"
    },
    {
        title: "总金额",
        key: "amount",
        width: 100,
        sorter: (row1, row2) => row1.amount - row2.amount
    }
]);

async function getRecordList() {
    let res = await invoke('get_pay_record_list', { 'name': username.value });
    res = JSON.parse(res);
    if (res.code === 0) {
        payRecordList.value = res.data;
    } else {
        console.log(res)
        message.error(res.msg);
    }
};

onMounted(() => {
    username.value = route.params.name
    getRecordList();
})

function goBack() {
    router.push({
        name: 'Main',
    })
}
</script>

<style></style>