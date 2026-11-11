<script setup lang="ts">
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { invoke } from '@tauri-apps/api/core';
    import { ref } from 'vue';

    const appWindow = getCurrentWindow();

    const bubble = ref<HTMLDivElement | null>(null);

    const isHidden = ref(false);

    setInterval(async () => {
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
    }, 50);
</script>

<template>
    <div class="center">
        <div class="bubble hidden" ref="bubble">
            
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

    .bubble {
        display: flex;
        position: relative;

        max-height: 48px;
        width: 320px;

        margin-top: 8px;

        border: 1px solid #AAA;
        border-radius: 16px;

        align-items: center;

        background-color: black;

        transition: transform 0.3s cubic-bezier(.11,.58,.3,.99);
    }

    .hidden {
        transform: translateY(-99.9%);
    }
</style>

<style>
    body {
        margin: 0;
    }
</style>