<script setup lang="ts">
    import BubbleMinimized from './BubbleMinimized.vue';
    import BubbleExpanded from './BubbleExpanded.vue';
    import { invoke } from '@tauri-apps/api/core';
    import { onMounted, onUnmounted, ref, watch } from 'vue';
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { isEqual } from 'lodash';

    const appWindow = getCurrentWindow();

    const bubble = ref<HTMLDivElement | null>(null); // A variable for Bubble element
    const isHidden = ref(false); // A state to whether the Bubble is showed
    const isOtherWindowHover = ref(false); // A state to whether Bubble is overlapped with other window
    
    // A function for fast show/hide a bubble
    async function setBubbleState(value: boolean) {
        bubble.value?.classList.toggle('hidden', !value);
        isHidden.value = !value;
        await appWindow.setIgnoreCursorEvents(!value);
    }

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
            mouseY >= 0 && mouseY <= (isHidden.value ? 1 : Number(bubbleRect?.bottom));

        if (isOverBubble && isHidden.value) {
            await setBubbleState(true);
        } else if (!isOverBubble && !isHidden.value) {
            await setBubbleState(false);
        }
    }

    // A function to check whether a Bubble is overlapped by window
    let bubbleOtherWindowCoverIntervalId: number | null = null;
    async function BubbleOtherWindowCoverInterval() {
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
        setBubbleState(!newValue);
    });

    onMounted(() => {
        bubbleOtherWindowCoverIntervalId = setInterval(BubbleOtherWindowCoverInterval, 200);
        bubbleMouseEffectId = setInterval(bubbleMouseEffect, 50);
        bubbleMouseHoverIntervalId = setInterval(bubbleMouseHoverInterval, 50);
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
    });
</script>

<template>
    <div class="center">
        <div class="bubble " ref="bubble">
            <BubbleMinimized />
            <BubbleExpanded />
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

        overflow: hidden;
        
        --cursor-x: 0px;
        --cursor-y: 0px;

        transition: transform 0.3s var(--anim),
        width 0.3s var(--anim);
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
</style>