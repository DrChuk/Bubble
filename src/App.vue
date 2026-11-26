<script setup lang="ts">
    import Bubble from './components/bubble/Bubble.vue';
    import { register, unregister } from '@tauri-apps/plugin-global-shortcut';
    import { useBubbleState } from './composables/useBubbleState';
    import { onMounted, onUnmounted } from 'vue';
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { useConfig } from './composables/useConfig';
    import { useHoverZones } from './composables/useHoverZones';
    import { invoke } from '@tauri-apps/api/core';

    const appWindow = getCurrentWindow();
    const {config, loadConfig} = useConfig();

    const { expand, hide, state } = useBubbleState();
    const { updateHover } = useHoverZones();

    let mouseIntervalId: number | null = null;
    async function mouseInterval() {
        const [mouseX, mouseY] = await invoke<[number, number]>('get_mouse_pos');

        updateHover(mouseX, mouseY);
    }

    onMounted(async () => {
        await loadConfig();

        await register(config.value.hotkey, (event) => {
            if (event.state === 'Pressed') {
                if (state.value === 'hidden') appWindow.setFocus();
                state.value === 'expanded' ? hide() : expand();
            }
        });

        mouseIntervalId = setInterval(mouseInterval, 50);
    });

    onUnmounted(async () => {
        await unregister(config.value.hotkey);

        if (mouseIntervalId !== null) {
            clearInterval(mouseIntervalId);
            mouseIntervalId = null;
        }
    });
</script>

<template>
    <Bubble />
</template>

<style>
    @import url('https://fonts.googleapis.com/css2?family=IBM+Plex+Sans+JP&display=swap');
    @import url('https://fonts.googleapis.com/css2?family=Quicksand');

    :root {
        --bubble-anim: cubic-bezier(0.35,1.3,0.64,1);
        --anim: cubic-bezier(.11,.58,.3,.99);
    }

    body {
        margin: 0;
    }
</style>
