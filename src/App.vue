<script setup lang="ts">
    import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { onMounted, onUnmounted, ref } from 'vue';

    const appWindow = getCurrentWindow();

    const bubble = ref<HTMLDivElement | null>(null);

    const isHidden = ref(false);

    let bubbleStateIntervalId: number | null = null;
    async function bubbleStateInterval() {
        const [mouseX, mouseY] = await invoke<[number, number]>('get_mouse_pos');

        const rect = bubble.value?.getBoundingClientRect();

        const isOverBubble = 
            mouseX >= Number(rect?.left) && mouseX <= Number(rect?.right) &&
            mouseY >= 0 && mouseY <= (isHidden.value ? 1 : Number(rect?.bottom));

        if (isOverBubble && isHidden.value) {
            isHidden.value = false;
            bubble.value?.classList.toggle('hidden', false);
        } else if (!isOverBubble && !isHidden.value) {
            isHidden.value = true;
            bubble.value?.classList.toggle('hidden', true);
        }

        await appWindow.setIgnoreCursorEvents(!isOverBubble);
    }

    const time = ref<string>('');
    const day = ref<string>('');
    const month = ref<string>('');
    const weekday = ref<string>('');

    let getTimeDateIntervalId: number | null = null;

    async function getTimeDateInterval() {
        time.value = await invoke<string>('get_time');
        const date = await invoke<string>('get_date');

        [weekday.value, day.value, month.value] = date.split(' ');
    }

    onMounted(() => {
        // bubbleStateIntervalId = setInterval(bubbleStateInterval, 50);
        getTimeDateIntervalId = setInterval(getTimeDateInterval, 100);
    });

    onUnmounted(() => {
        if (bubbleStateIntervalId !== null) {
            clearInterval(bubbleStateIntervalId);
            bubbleStateIntervalId = null;
        }
        if (getTimeDateIntervalId !== null) {
            clearInterval(getTimeDateIntervalId);
            getTimeDateIntervalId = null;
        }
    });
</script>

<template>
    <div class="center">
        <div class="bubble " ref="bubble">
            <p class="time">{{ time }}</p>
            <div class="date">
                
            </div>
        </div>
    </div>
</template>

<style scoped>
    *, *::after, *::before {
        box-sizing: border-box;
        user-select: none;
    }

    .center {
        display: flex;
        position: relative;

        width: 100%;
        height: 100dvh;

        justify-content: center;

        pointer-events: none;
    }

    .bubble {
        display: flex;
        position: relative;

        flex-flow: column;
        justify-content: center;
        align-items: center;

        max-height: 40px;
        width: 320px;

        margin-top: 8px;

        background-color: black;
        border-radius: 40px;
        border: 1px solid #777;

        transition: transform 0.3s cubic-bezier(.11,.58,.3,.99);
    }

    .bubble p {
        font-family: 'IBM Plex Sans JP', sans-serif;
        margin: 0;
        font-size: 12px;
        width: 100%;

        color: white;
    }

    .time {
        padding-left: 12px;
    }

    .hidden {
        transform: translateY(-99%);
    }
</style>

<style>
    @import url('https://fonts.googleapis.com/css2?family=IBM+Plex+Sans+JP&display=swap');

    body {
        margin: 0;
    }
</style>