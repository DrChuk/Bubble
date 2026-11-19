<script setup lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { onMounted, onUnmounted, ref } from 'vue';

    const props = defineProps<{
        id: string,
        title: string,
        icon: string,
        component: string
    }>();

    const appItemRef = ref<HTMLDivElement | null>(null);

    let hoverIntervalId: number | null = null;
    async function hoverInterval() {
        const [mouseX, mouseY] = await invoke<[number, number]>('get_mouse_pos');
        const appItemRect = appItemRef.value?.getBoundingClientRect();

        const isHoverAppItem = 
            mouseX >= Number(appItemRect?.left) && mouseX <= Number(appItemRect?.right) &&
            mouseY >= Number(appItemRect?.top) && mouseY <= Number(appItemRect?.bottom);

        appItemRef.value?.classList.toggle('hover', isHoverAppItem);
    }

    onMounted(() => {
        hoverIntervalId = setInterval(hoverInterval, 50);
    })

    onUnmounted(() => {
        if (hoverIntervalId !== null) {
            clearInterval(hoverIntervalId);
            hoverIntervalId = null;
        }
    })
</script>

<template>
    <div class="background" ref="appItemRef">
        <span
            class="icon"
            :style="{
                maskImage: `url(${props.icon})`
            }"
        />
    </div>
</template>

<style scoped>
    .icon {
        position: relative;

        width: 28px;
        height: 28px;

        background-color: white;

        mask-size: contain;
        -webkit-mask-size: contain;
        mask-repeat: no-repeat;
        -webkit-mask-reaeat: no-repeat;
        mask-position: center;

        pointer-events: none;
    }

    .background {
        display: flex;
        position: relative;

        justify-content: center;
        align-items: center;

        width: 32px;
        height: 32px;
        border-radius: 5px;

        background-color: rgba(255, 255, 255, 0.3);
        border: 1px solid rgba(255, 255, 255, 0.4);
        backdrop-filter: blur(20px);

        transition: transform 0.3s var(--anim);
    }

    .background.hover {
        transform: translateY(-10%) scale(120%, 120%);
    }
</style>
