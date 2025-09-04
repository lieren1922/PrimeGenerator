<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { message } from "@tauri-apps/api/dialog";

const start = ref();
const end = ref();
const primeList = ref<number[]>([]);

const waitingResFlag = ref(false);

const checkRange = async () => {
    if (start.value >= 1 && end.value >= start.value) {
        return true;
    } else if (start.value < 1) {
        await message("Start mast greater than 0.", {
            title: "Error",
            type: "error",
        });
    } else if (start.value > end.value) {
        await message("End mast greater than or equal to start.", {
            title: "Error",
            type: "error",
        });
    }
    start.value = "";
    end.value = "";
    primeList.value = [];
    return false;
};

const genPrimes = async () => {
    let checkRes = await checkRange();
    if (checkRes) {
        waitingResFlag.value = true;
        let res: number[] = await invoke("yield_prime_range", {
            start: start.value,
            end: end.value,
        });
        primeList.value = res;
        waitingResFlag.value = false;
    }
};
</script>

<template>
    <div class="info">
        <span class="info1">Prime Generator</span>
        <span class="info2">Powered by Tauri</span>
    </div>
    <div class="input-button">
        <div class="input-box">
            <input
                name="text"
                class="input"
                placeholder="start"
                v-model.number="start"
            />
            <span class="span">&nbsp; ~ &nbsp;</span>
            <input
                name="text"
                class="input"
                placeholder="end"
                v-model.number="end"
            />
        </div>
        <button
            class="genPrime"
            @click="genPrimes"
            :class="{ waiting: waitingResFlag }"
        >
            Generate
        </button>
    </div>

    <div class="showBox">
        <div class="scroll-container">
            <li v-for="prime in primeList">{{ prime }}</li>
        </div>
    </div>
</template>

<style scoped>
.info {
    display: flex;
    flex-direction: column;
    justify-content: center;
    height: 50px;
    margin-bottom: 20px;
}
.info1 {
    font-size: 2em;
    color: rgb(25, 25, 25);
    text-shadow:
        1px 1px 1px black,
        -1px -1px 2px #888;
}
.info2 {
    color: rgb(25, 25, 25);
    text-shadow:
        1px 1px 1px black,
        -1px -1px 2px #888;
}

.span {
    color: rgb(25, 25, 25);
    text-shadow:
        1px 1px 1px black,
        -1px -1px 2px #888;
}
.showBox {
    margin-top: 20px;
    margin-bottom: -20px;
    display: flex;
    justify-content: space-around;
}

.input {
    height: 40px;
    width: 80px;
    border: none;
    outline: none;
    padding: 0px 7px;
    border-radius: 6px;
    color: #bbb;
    font-size: 15px;
    background-color: transparent;
    box-shadow:
        3px 3px 10px rgba(0, 0, 0, 1),
        -1px -1px 6px rgba(255, 255, 255, 0.4);
}
.input:focus {
    border: 0px solid transparent;
    color: #fff;
    box-shadow:
        3px 3px 10px rgba(0, 0, 0, 1),
        -1px -1px 6px rgba(255, 255, 255, 0.4),
        inset 3px 3px 10px rgba(0, 0, 0, 1),
        inset -1px -1px 6px rgba(255, 255, 255, 0.4);
}
.input-box {
    height: 45px;
    display: flex;
    justify-content: center;
    align-items: center;
}

.input::-webkit-input-placeholder {
    font-size: 12px;
    color: #888;
}

.input-button {
    height: 130px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-around;
    gap: 7px;
    position: relative;
    color: white;
}
.scroll-container {
    text-align: center;
    background-color: rgb(26, 24, 40);
    color: #bbb;
    border-radius: 10px;
    border-width: 5px;
    font-size: 15px;
    width: 200px;
    height: 120px; /* 设置滚动框的高度 */
    overflow-y: auto; /* 允许垂直滚动 */
    list-style-type: none;

    background: #212121;
    border: 1px solid #212121;
    transition: all 0.3s;
    box-shadow:
        3px 3px 10px rgba(0, 0, 0, 1),
        -1px -1px 6px rgba(255, 255, 255, 0.4);
}

/* 自定义滚动条样式 */
.scroll-container::-webkit-scrollbar {
    width: 8px; /* 设置滚动条的宽度 */
}
.scroll-container::-webkit-scrollbar-track {
    background: rgb(25, 25, 25);
    border-radius: 5px; /* 设置滚动轨道的圆角 */
}

.scroll-container::-webkit-scrollbar-thumb {
    background: #555;
    border-radius: 2px; /* 设置滚动滑块的圆角 */
}
.scroll-container::-webkit-scrollbar-thumb:hover {
    background: #555;
}

.genPrime {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    width: 150px;
    margin-top: -6px;
    height: 45px;
    color: #888;
    padding: 0.7em 1.7em;
    font-size: 18px;
    border-radius: 0.5em;
    background: #212121;
    border: 1px solid #212121;
    transition: all 0.3s;
    box-shadow:
        3px 3px 10px rgba(0, 0, 0, 1),
        -1px -1px 6px rgba(255, 255, 255, 0.4);
    position: relative; /* 为灯定位 */
}

/* 定义灯的位置和大小 */
.genPrime:before {
    content: "";
    position: absolute;
    top: 5px;
    left: 5px;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background-color: green; /* 默认的绿色 */
    transition: background-color 0.3s; /* 平滑过渡颜色变化 */
}
/* 当按钮处于等待状态时的灯颜色 */
.genPrime.waiting:before {
    background-color: red; /* 等待时的红色 */
}

button:active {
    color: #666;
    box-shadow:
        inset 4px 4px 12px #000,
        inset -4px -4px 12px #1f1f1f;
}
</style>
