<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { message } from "@tauri-apps/api/dialog";

// const msg = "msg from tauri...";
// const msgList = ref<string[]>([]);

const prime_nums = ref();
const primeList = ref<number[]>([]);

// const getMsg = async () => {
//     let res: string = await invoke("yield_msg");
//     msgList.value.unshift(res);
// };

// watch(prime_nums, async (newVal, _oldVal) => {
//   let res: number[] = await invoke("yield_prime", {nums: newVal})
//   primeList.value = res;
// });

const genPrimes = async () => {
    if (prime_nums.value < 1) {
        await message("prime numbers mast greater than 0", {
            title: "Error",
            type: "error",
        });
        prime_nums.value = "";
        primeList.value = [];
    }
    let res: number[] = await invoke("yield_prime", { nums: prime_nums.value });
    primeList.value = res;
};
</script>

<template>
    <!-- <button @click="getMsg">click to get a message!</button>
    <br />
    <li v-for="msg in msgList">{{ msg }}</li> -->

    <h2>Prime Generator</h2>

    <div class="input-button">
        <div class="input-box">
            <input
                name="text"
                class="input"
                placeholder="Please enter the prime numbers up to n."
                v-model.number="prime_nums"
            />
        </div>
        <button class="genPrime" @click="genPrimes">Generate</button>
    </div>

    <div class="showBox">
        <div class="scroll-container">
            <li v-for="prime in primeList">{{ prime }}</li>
        </div>
    </div>
</template>

<style scoped>
h2 {
    font-size: 2em;
    color: rgb(25, 25, 25);
    text-shadow:
        1px 1px 1px black,
        -1px -1px 2px #888;
}

.showBox {
    margin-top: 20px;
    display: flex;
    justify-content: space-around;
}

.input {
    width: 230px;
    height: 40px;
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
}
button:active {
    color: #666;
    box-shadow:
        inset 4px 4px 12px #000,
        inset -4px -4px 12px #1f1f1f;
}
</style>
