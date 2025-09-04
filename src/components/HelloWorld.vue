<script setup lang="ts">
import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";

const count = ref(0);
defineProps<{ msg: string }>();

const emit_msg = async () => {
    emit("click", {
        theMessage: "Tauri is awesome!",
    });
    await listen("click", (event) => {
        console.log(`the event is ${event.event}`);
        console.log(`the msg is ${event.payload}`);
    });
};

</script>

<template>
    <h1>{{ msg }}</h1>

    <div class="card">
        <button type="button" @click="count++">count is {{ count }}</button>
        <button type="button" @click="emit_msg">emit_msg</button>
    </div>
</template>
