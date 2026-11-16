<script setup lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { ref, onMounted, onUnmounted } from "vue";

    const container = ref<HTMLDivElement | null>(null);

    let mouseEffectIntervalId: number | null = null;
    async function mouseEffectInterval() {
        let [mouseX, mouseY] = await invoke<[number, number]>('get_mouse_pos');

        container.value?.style.setProperty("--cursor-x", `${mouseX}px`);
        container.value?.style.setProperty("--cursor-y", `${mouseY}px`);
    }

    onMounted(() => {
        mouseEffectIntervalId = setInterval(mouseEffectInterval, 50);
    }); 

    onUnmounted(() => {
        if (mouseEffectIntervalId !== null) {
            clearInterval(mouseEffectIntervalId);
            mouseEffectIntervalId = null;
        }
    })
</script>

<template>
    <div class="container" ref="container">
        
    </div>
</template>

<style scoped>
    .container {
        display: flex;
        position: relative;

        width: 480px;
        height: 370px;

        overflow: hidden;

        --cursor-x: 0px;
        --cursor-y: 0px;
    }

    .container::after {
        content: "";
        position: absolute;

        top: 0;
        left: 0;

        width: 100px;
        height: 100px;
        background-color: white;
        border-radius: 100px;
        filter: blur(100px);

        /* transform: translate(calc(50% - var(--cursor-x) * .1), calc(50% - var(--cursor-y) * .1)); */
        transform: translate(calc((240px - 50%) + (var(--cursor-x) - 240px) * .1),
            calc((185px - 50%) + (var(--cursor-y) - 185px) * .1));
    }
</style>
