<script setup lang="ts">
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { invoke } from '@tauri-apps/api/core';
    import { ref } from 'vue';

    const appWindow = getCurrentWindow();

    const indicator = ref<HTMLSpanElement | null>(null);
    const bubble = ref<HTMLDivElement | null>(null);

    setInterval(async () => {
        const [mouseX, mouseY] = await invoke<[number, number]>('get_mouse_pos');

        const rect = bubble.value?.getBoundingClientRect();
        const isOverBubble = 
            mouseX >= Number(rect?.left) && mouseX <= Number(rect?.right) &&
            mouseY >= Number(rect?.top) && mouseY <= Number(rect?.bottom);

        console.log('X: ' + mouseX + ', Y: ' + mouseY + ', isOver: ' + isOverBubble);

        indicator.value?.classList.toggle('on', isOverBubble);
        await appWindow.setIgnoreCursorEvents(!isOverBubble);
    }, 50);
</script>

<template>
    <div class="center">
        <div class="bubble" ref="bubble">
            <span class="indicator" ref="indicator"></span>
        </div>
    </div>
</template>

<style scoped>
    *, *::after, *::before {
        box-sizing: border-box;
    }

    .center {
        display: flex;
        position: relative;

        width: 100%;
        height: 100dvh;

        justify-content: center;

        pointer-events: none;
    }

    .indicator {
        position: relative;

        width: 12px;
        height: 12px;

        margin-left: 12px;

        border-radius: 12px;
        background-color: #f44;

        transition: background-color 0.2s ease-out;
    }

    .on {
        background-color: #4f4;
    }

    .bubble {
        display: flex;
        position: relative;

        max-height: 64px;
        width: 320px;

        margin-top: 8px;

        border: 1px solid #AAA;
        border-radius: 16px;

        align-items: center;

        background-color: black;
    }
</style>

<style>
    body {
        margin: 0;
    }
</style>