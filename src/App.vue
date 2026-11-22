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
