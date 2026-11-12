<script setup lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { onMounted, onUnmounted, ref, watch } from 'vue';
    import { isEqual } from 'lodash';

    const appWindow = getCurrentWindow();

    const bubble = ref<HTMLDivElement | null>(null);
    const isHidden = ref(false);
    const isOtherWindowHover = ref(false);
    
    async function setBubbleState(value: boolean) {
        bubble.value?.classList.toggle('hidden', !value);
        isHidden.value = !value;
        await appWindow.setIgnoreCursorEvents(!value);
    }

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

    let bubbleOtherWindowHoverIntervalId: number | null = null;
    async function bubbleOtherWindowHoverInterval() {
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

    const time = ref<string>('');
    const day = ref<string>('');
    const month = ref<string>('');
    const weekday = ref<string>('');

    let getTimeDateIntervalId: number | null = null;

    async function getTimeDateInterval() {
        time.value = await invoke<string>('get_time');
        const date = await invoke<string>('get_date');

        [weekday.value, day.value, month.value] = date.toUpperCase().split(' ');
    }

    onMounted(() => {
        bubbleOtherWindowHoverIntervalId = setInterval(bubbleOtherWindowHoverInterval, 200);
        bubbleMouseEffectId = setInterval(bubbleMouseEffect, 50);
        bubbleMouseHoverIntervalId = setInterval(bubbleMouseHoverInterval, 50);
        getTimeDateIntervalId = setInterval(getTimeDateInterval, 100);
    });

    onUnmounted(() => {
        if (bubbleOtherWindowHoverIntervalId !== null) {
            clearInterval(bubbleOtherWindowHoverIntervalId);
            bubbleOtherWindowHoverIntervalId = null;
        }
        if (bubbleMouseEffectId !== null) {
            clearInterval(bubbleMouseEffectId);
            bubbleMouseEffectId = null;
        }
        if (bubbleMouseHoverIntervalId !== null) {
            clearInterval(bubbleMouseHoverIntervalId);
            bubbleMouseHoverIntervalId = null;
        }
        if (getTimeDateIntervalId !== null) {
            clearInterval(getTimeDateIntervalId);
            getTimeDateIntervalId = null;
        }
    });
</script>

<template>
    <div class="mini-bubble">
        <div class="bubble " ref="bubble">
            <p class="time">{{ time }}</p>
            <div class="date">
                <p class="weekday">{{ weekday }}</p>
                <span class="dot"></span>
                <p class="day">{{ day }}</p>
                <p class="month">{{ month }}</p>
            </div>
            <div class="more"></div>
        </div>
    </div>
</template>

<style scoped>
    *, *::after, *::before {
        box-sizing: border-box;
        user-select: none;
    }

    .mini-bubble {
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

    .bubble p {
        position: relative;
        font-family: 'IBM Plex Sans JP', sans-serif;
        margin: 0;

        pointer-events: none;
    }

    .bubble p:hover {
        color: red;
    }

    .bubble div {
        width: 100%;
    }
    
    .bubble .dot {
        position: relative;
        width: 3px;
        height: 3px;

        border-radius: 5px;
        background-color: #555;

        margin: 0 4px;
    }

    .date {
        display: flex;
        position: relative;

        align-items: center;
        gap: 3px;
    }

    .weekday {
        color: #aaa;

        font-size: 11px;
    }

    .day {
        color: white;
        font-size: 13px;
    }

    .month {
        color: #aaa;
        font-size: 11px;
    }

    .time {
        padding-left: 12px;
        font-size: 12px;
        width: 100%;

        color: white;
    }
</style>

<style>
    @import url('https://fonts.googleapis.com/css2?family=IBM+Plex+Sans+JP&display=swap');

    :root {
        --anim: cubic-bezier(.11,.58,.3,.99);
    }

    body {
        margin: 0;
    }
</style>