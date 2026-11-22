<script setup lang="ts">
    import Bubble from './components/bubble/Bubble.vue';
    import { register, unregister } from '@tauri-apps/plugin-global-shortcut';
    import { useBubbleState } from './composables/useBubbleState';
    import { onMounted, onUnmounted } from 'vue';
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { useConfig } from './composables/useConfig';

    const appWindow = getCurrentWindow();
    const {config, loadConfig} = useConfig();

    const { expand, hide, state } = useBubbleState();

    onMounted(async () => {
        await loadConfig();

        await register(config.value.hotkey, (event) => {
            if (event.state === 'Pressed') {
                if (state.value === 'hidden') appWindow.setFocus();
                state.value === 'expanded' ? hide() : expand();
            }
        });
    });

    onUnmounted(async () => {
        await unregister(config.value.hotkey);
    });
</script>

<template>
    <Bubble />
</template>

<style>
    @import url('https://fonts.googleapis.com/css2?family=IBM+Plex+Sans+JP&display=swap');

    :root {
        --bubble-anim: cubic-bezier(0.3, -0.2, 0, 1.25);
        --anim: cubic-bezier(.11,.58,.3,.99);
    }

    body {
        margin: 0;
    }
</style>
