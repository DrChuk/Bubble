<script setup lang="ts">
    import { useBubbleState } from '../../composables/useBubbleState.ts';
    import BubbleHidden from './BubbleHidden.vue';
    import BubbleMinimized from './BubbleMinimized.vue';
    import BubbleExpanded from './BubbleExpanded.vue';
    import { invoke } from '@tauri-apps/api/core';
    import { onMounted, onUnmounted, ref, watch } from 'vue';
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { isEqual } from 'lodash';

    const appWindow = getCurrentWindow();

    const { state } = useBubbleState();

    const bubble = ref<HTMLDivElement | null>(null); // A variable for Bubble element
    const isOtherWindowHover = ref(false); // A state to whether Bubble is overlapped with other window
    
    // A function for fast show/hide a bubble
    watch(state, async (newValue) => {
        bubble.value?.classList.toggle('hidden', newValue === 'hidden');
        await appWindow.setIgnoreCursorEvents(newValue === 'hidden');
    });

    // A function for a cursor-following effect
    let bubbleMouseEffectId: number | null = null;
    async function bubbleMouseEffect() {
        const [mouseX, mouseY] = await invoke<[number, number]>('get_mouse_pos');
        const bubbleRect = bubble.value?.getBoundingClientRect();

        const isHoverBubble = 
            mouseX >= Number(bubbleRect?.left) && mouseX <= Number(bubbleRect?.right) &&
            mouseY >= Number(bubbleRect?.top) && mouseY <= Number(bubbleRect?.bottom);

        bubble.value?.classList.toggle('hover', isHoverBubble);

        if (isHoverBubble) {
            bubble.value?.style.setProperty('--cursor-x', `${mouseX}px`);
            bubble.value?.style.setProperty('--cursor-y', `${mouseY}px`);
        }
    }
    
    // A function to check whether a Bubble hovered
    let bubbleMouseHoverIntervalId: number | null = null;
    async function bubbleMouseHoverInterval() {
        if (!isOtherWindowHover.value) return;
        const [mouseX, mouseY] = await invoke<[number, number]>('get_mouse_pos');
        const bubbleRect = bubble.value?.getBoundingClientRect();

        const isOverBubble = 
            mouseX >= Number(bubbleRect?.left) && mouseX <= Number(bubbleRect?.right) &&
            mouseY >= 0 && mouseY <= (state.value === 'hidden' ? 1 : Number(bubbleRect?.bottom));

        if (isOverBubble && state.value === 'hidden') {
            state.value = 'minimized';
        } else if (!isOverBubble && state.value !== 'hidden') {
            state.value = 'hidden';
        }
    }

    // A function to check whether a Bubble is overlapped by window
    let bubbleOtherWindowCoverIntervalId: number | null = null;
    async function bubbleOtherWindowCoverInterval() {
        const windowRect = await invoke<[number, number, number, number]>('get_absolute_window_rect');
        const otherWindowRect = await invoke<[number, number, number, number]>('get_foreground_window_rect');

        if (isEqual(windowRect, otherWindowRect)) {
            isOtherWindowHover.value = false;
            return;
        }

        const isIntersect = await invoke<boolean>('rects_intersect',
            {
                a: [
                    windowRect[0],
                    windowRect[1],
                    windowRect[2],
                    windowRect[3]
                ],
                b: [
                    otherWindowRect[0],
                    otherWindowRect[1],
                    otherWindowRect[2],
                    otherWindowRect[3]
                ]
        })

        isOtherWindowHover.value = isIntersect;
    }

    watch(isOtherWindowHover, async (newValue) => {
        state.value = newValue ? 'hidden' : 'minimized';
    });

    let updateBorderColorId: number | null = null;
    async function updateBorderColor() {
        const windowRect = await invoke<[number, number, number, number]>('get_absolute_window_rect');
        const yPixel = windowRect[1] + 5;
        const firstPixel = await invoke<[number, number, number]>('get_pixel', {x: windowRect[0] - 20, y: yPixel});
        const secondPixel = await invoke<[number, number, number]>('get_pixel', {x: windowRect[2] + 20, y: yPixel});

        const firstPixelIsDark = await invoke<boolean>('is_dark', {r: firstPixel[0], g: firstPixel[1], b: firstPixel[2]});
        const secondPixelIsDark = await invoke<boolean>('is_dark', {r: secondPixel[0], g: secondPixel[1], b: secondPixel[2]});

        bubble.value?.classList.toggle('border', firstPixelIsDark && secondPixelIsDark && state.value === 'hidden');
    }

    onMounted(() => {
        bubbleOtherWindowCoverIntervalId = setInterval(bubbleOtherWindowCoverInterval, 200);
        bubbleMouseEffectId = setInterval(bubbleMouseEffect, 50);
        bubbleMouseHoverIntervalId = setInterval(bubbleMouseHoverInterval, 50);
        updateBorderColorId = setInterval(updateBorderColor, 100);
    });

    onUnmounted(() => {
        if (bubbleOtherWindowCoverIntervalId !== null) {
            clearInterval(bubbleOtherWindowCoverIntervalId);
            bubbleOtherWindowCoverIntervalId = null;
        }
        if (bubbleMouseEffectId !== null) {
            clearInterval(bubbleMouseEffectId);
            bubbleMouseEffectId = null;
        }
        if (bubbleMouseHoverIntervalId !== null) {
            clearInterval(bubbleMouseHoverIntervalId);
            bubbleMouseHoverIntervalId = null;
        }
        if (updateBorderColorId !== null) {
            clearInterval(updateBorderColorId);
            updateBorderColorId = null;
        }
    });
</script>

<template>
    <div class="center">
        <div class="bubble hidden" ref="bubble">
                <BubbleHidden class="layer" :class="{'layer-active': state === 'hidden'}" />
                <BubbleMinimized class="layer" :class="{'layer-active': state === 'minimized'}" />
                <BubbleExpanded class="layer" :class="{'layer-active': state === 'expanded'}" />
        </div>
    </div>

</template>

<style scoped>
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

        flex-flow: row;
        justify-content: center;
        align-items: center;

        max-height: 40px;
        width: 320px;

        margin-top: 8px;

        background-color: black;
        border-radius: 40px;
        outline: 1px solid transparent;

        overflow: hidden;
        
        --cursor-x: 0px;
        --cursor-y: 0px;

        transition: transform 0.3s var(--anim),
        width 0.3s var(--anim),
        outline 0.3s var(--anim);
    }

    .bubble.border {
        outline-color: #333;
    }

    .layer {
        opacity: 0;
        visibility: hidden;
        pointer-events: none;
        transition: opacity 0.3s var(--anim), visibility 0.3s var(--anim);

        position: absolute;
        inset: 0;
    }

    .layer-active {
        opacity: 1;
        visibility: visible;
        pointer-events: auto;
        position: relative;
    }

    .hidden {
        width: 300px;
        transform: translateY(-99%);
    }

    .bubble::after {
        content: "";
        position: absolute;
        top: 0;
        left: 0;

        width: 100px;
        height: 100px;
        border-radius: 50px;
        
        background-color: black;
        filter: blur(100px);

        transform: translate(calc(var(--cursor-x) - 50% - 50%), calc(var(--cursor-y) - 50%));

        transition: background-color 0.5s var(--anim);
    }

    .bubble.hover::after {
        background-color: white;
    }
</style>

<style>
    *, *::after, *::before {
        box-sizing: border-box;
        user-select: none;
    }
    .container {
        display: flex;
        position: relative;
        width: 100%;
        height: 100%;

        align-items: center;
        justify-content: center;
    }
</style>