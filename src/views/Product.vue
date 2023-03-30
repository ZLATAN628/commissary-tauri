<template>
    <n-form ref="formRef" :model="model" :rules="rules" label-placement="left" label-width="auto"
        require-mark-placement="right-hanging" :size="size" :style="{
            maxWidth: '640px'
        }">
        <n-form-item label="商品名称" path="product_name">
            <n-input v-model:value="model.product_name" placeholder="Input" />
        </n-form-item>
        <n-form-item label="商品类型" path="product_type">
            <n-select v-model:value="model.product_type" placeholder="Select" :options="generalOptions" />
        </n-form-item>
        <n-form-item label="采购单价" path="cost">
            <n-input-number v-model:value="model.cost" />
        </n-form-item>
        <n-form-item label="采购数量" path="count">
            <n-input-number v-model:value="model.count" />
        </n-form-item>
        <n-form-item label="售卖单价" path="price">
            <n-input-number v-model:value="model.price" />
        </n-form-item>
        <n-form-item label="上传商品图片">
            <n-upload :data="model.image" name="image" :on-update:file-list="fileChange" ref="imageRef">
                <n-button>上传文件</n-button>
            </n-upload>
        </n-form-item>
        <div style="text-align: center;">
            <n-button round type="primary" @click="handleValidateButtonClick">
                提交
            </n-button>
            <n-button type="warning" @click="cancelClick" style="margin-left: 30px;">
                取消
            </n-button>
        </div>
    </n-form>
</template>
  
<script setup>
import { ref } from "vue";
import { NForm, NFormItem, NButton, NUpload, NInput, NSelect, NInputNumber, useMessage } from "naive-ui";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/tauri";

const formRef = ref(null);
const message = useMessage();

const model = ref({
    product_name: null,
    product_type: null,
    cost: null,
    count: null,
    price: null,
    image: null,
    owner: '张建华'
});

const generalOptions =
    [
        {
            label: "干货",
            value: 1
        },
        {
            label: "饮料",
            value: 2
        },
        {
            label: "小俞咖啡",
            value: 3
        },
        {
            label: "其他",
            value: 4
        },
    ];

const rules = {
    product_name: {
        required: true,
        trigger: ["blur", "input"],
        message: "请输入商品名称"
    },
    cost: {
        type: "number",
        required: true,
        trigger: ["blur", "change"],
        message: "请输入采购单价"
    },
    count: {
        type: "number",
        required: true,
        trigger: ["blur", "change"],
        message: "请输入采购数量"
    },
    price: {
        type: "number",
        required: true,
        trigger: ["blur", "change"],
        message: "请输入销售单价"
    },
};

const router = useRouter();

let imageRef = ref();


function handleValidateButtonClick(e) {
    e.preventDefault();
    formRef.value?.validate((errors) => {
        if (!errors) {
            if (!model.value.image) {
                alert("请上传商品图片")
                return;
            }
            invoke("insert_product", { 'data': JSON.stringify(model.value, null, 2) }).then(e => {
                let res = JSON.parse(e);
                if (res.code == 0) {
                    message.success("新增成功")
                    model.value = {
                        product_name: null,
                        product_type: null,
                        cost: null,
                        count: null,
                        price: null,
                        image: null,
                        owner: '张建华'
                    }
                    imageRef.value.clear();
                } else {
                    message.error(res.msg)
                }
            }).catch(e => {
                message.error("新增失败")
            });
        }
    });
}

function cancelClick(e) {
    router.push({
        name: "Main",
    });
}

function fileChange(file) {
    if (file && file.length > 0) {
        let reader = new FileReader();
        reader.readAsDataURL(file[0].file);
        reader.onload = () => {
            model.value.image = reader.result;
        }
    }
}
</script>