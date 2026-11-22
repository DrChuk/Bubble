<script setup lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { onMounted, onUnmounted, ref } from 'vue';
    import { useMenuRegistry } from '../../composables/useMenuRegistry';

    const props = defineProps<{
        id: string,
        title: string,
        icon: string,
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

    const {setActive, activeId} = useMenuRegistry();

    let isActiveAppId: number | null = null;

    function isActiveApp() {
        appItemRef.value?.classList.toggle('active', activeId.value === props.id);
    }

   function onClick() {
        setActive(props.id);
    }

    onMounted(() => {
        hoverIntervalId = setInterval(hoverInterval, 50);
        isActiveAppId = setInterval(isActiveApp, 50);
    })

    onUnmounted(() => {
        if (hoverIntervalId !== null) {
            clearInterval(hoverIntervalId);
            hoverIntervalId = null;
        }
        if (isActiveAppId !== null) {
            clearInterval(isActiveAppId);
            isActiveAppId = null;
        }
    })
</script>

<template>
    <div class="background" ref="appItemRef" @click="onClick">
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

        background-color: rgba(255, 255, 255, 0.1);
        border: 1px solid rgba(255, 255, 255, 0.2);

        transform: translateZ(0);
        transition: transform 0.3s var(--anim);
    }

    .background.hover:not(.active) {
        will-change: transform;
        transform: translateZ(0) translateY(-10%) scale(120%, 120%);
    }

    .background::after {
        content: "";
        position: absolute;

        width: 4px;
        height: 4px;
        bottom: -30%;
        left: 50%;

        border-radius: 4px;

        background-color: transparent;
        transform: translateX(-50%);
        transition: background-color 0.3s var(--anim);
    }

    .background.active::after {
        background-color: white;
    }
</style>
