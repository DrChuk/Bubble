<script setup lang="ts">
    import Bubble from './components/bubble/Bubble.vue';
    import { register, unregister } from '@tauri-apps/plugin-global-shortcut';
    import { useBubbleState } from './composables/useBubbleState';
    import { onMounted, onUnmounted } from 'vue';

    const { expand, hide, state } = useBubbleState();

    // await register('Super+Alt+Q', (event) => {
    //     if (event.state === 'Pressed') {
    //         expand();
    //     }
    // });
    onMounted(async () => {
        await register('Super+Alt+Q', (event) => {
            if (event.state === 'Pressed') {
                state.value === 'expanded' ? hide() : expand();
            }
        });
    });

    onUnmounted(async () => {
        await unregister('Super+Alt+Q');
    });
</script>

<template>
    <Bubble />
</template>
