<template>
    <Carousel />

    <n-space vertical>
        <n-data-table :columns="columns" :data="payRecordList" min-height="322px" flex-height striped
            :single-line="false" />
    </n-space>



    <div style="text-align: center;width: 100%;margin-top: 5px;display: inline-block;">
        <n-button color="#fcf4df" text-color="#397971" round @click="changeType"
            style="font-family:方正舒体;font-size: larger;width: 40%;margin-top: 10px;" strong="true">
            {{ text }}
        </n-button>
        <n-button color="#fcf4df" text-color="#397971" round @click="goBack"
            style="font-family:方正舒体;font-size: larger;width: 40%;margin-top: 10px;" strong="true">
            返回主页
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
const text = ref("查看总榜")

const payRecordList = ref([]);
const columns = ref([
    {
        title: "No",
        key: "no",
        width: 80,
        sorter: (row1, row2) => row1.no - row2.no
    },
    {
        title: "姓名",
        key: "customer_name",
        width: 160,
    },
    {
        title: "总金额",
        key: "amount",
        width: 160,
        sorter: (row1, row2) => row1.amount - row2.amount
    }
]);

async function getRecordList(type = 1) {
    // record_type = 1 月榜 record_type = 2 总榜
    let res = await invoke('get_total_record_list', { 'recordType': type });
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

function changeType() {
    if (text.value === "查看总榜") {
        text.value = "查看月榜";
        getRecordList(2);
    } else {
        text.value = "查看总榜";
        getRecordList();
    }
}
</script>

<style></style>