<script setup lang="ts">
    import AppDock from "./../appdock/AppDock.vue";
    import { useBubbleState } from "../../composables/useBubbleState";
    import { invoke } from "@tauri-apps/api/core";
    import { onMounted, onUnmounted, ref } from "vue";
    import { useMenuRegistry } from "../../composables/useMenuRegistry";

    const { state } = useBubbleState();
    const minimizeBtn = ref<HTMLButtonElement | null>(null);

    let onMouseHoverId: number | null = null;
    async function onMouseHover() {
        const [mouseX, mouseY] = await invoke<[number, number]>('get_mouse_pos');
        const rect = minimizeBtn.value?.getBoundingClientRect();

        const isHover = 
            mouseX >= Number(rect?.left) && mouseX <= Number(rect?.right) &&
            mouseY >= Number(rect?.top) && mouseY <= Number(rect?.bottom);

        minimizeBtn.value?.classList.toggle('hover', isHover);
    }

    function onMinimizeClick() {
        state.value = 'minimized';
    }

    const {activeApp} = useMenuRegistry();

    onMounted(() => {
        onMouseHoverId = setInterval(onMouseHover, 50);
    })

    onUnmounted(() => {
        if (onMouseHoverId !== null) {
            clearInterval(onMouseHoverId);
            onMouseHoverId = null;
        }
    });
</script>

<template>
    <div class="container">
        <div class="center">
            <button class="minimize" @click.stop="onMinimizeClick">
                <span class="icon"></span>
            </button>
        </div>
        <div class="appcard">
            <component :is="activeApp?.component" v-if="activeApp"></component>
        </div>
       <AppDock />
    </div>
</template>

<style scoped>
    .container {
        display: flex;
        position: relative;

        flex-direction: column;
        justify-content: center;

        width: 480px;
        height: 370px;

        overflow: hidden;

        background: linear-gradient(
            45deg,
            #222,
            #000
        );
    }

    .appcard {
        width: 100%;
        height: 100%;
    }

    .center {
        display: flex;
        position: relative;

        width: 100%;
        height: auto;

        justify-content: center;
        padding-top: 8px;
    }
    
    .minimize {
        display: flex;
        position: relative;
 
        width: 36px;
        height: 36px;

        background-color: transparent;

        border: none;
        border-radius: 36px;

        transition: background-color 0.3s var(--anim);
    }

    .minimize:hover {
        background-color: rgba(255, 255, 255, 0.05);
        cursor: pointer;
    }

    .minimize .icon {
        position: relative;

        width: 36px;
        height: 36px;

        background-color: white;

        mask: url("../../assets/chevron-up.svg");
        mask-size: contain;
        mask-repeat: no-repeat;
        mask-position: center;
    }
</style>
