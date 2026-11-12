<script setup lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { onMounted, onUnmounted, ref } from 'vue';

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
        getTimeDateIntervalId = setInterval(getTimeDateInterval, 100);
    });

    onUnmounted(() => {
        if (getTimeDateIntervalId !== null) {
            clearInterval(getTimeDateIntervalId);
            getTimeDateIntervalId = null;
        }
    });
</script>

<template>
    <p class="time">{{ time }}</p>
    <div class="date">
        <p class="weekday">{{ weekday }}</p>
        <span class="dot"></span>
        <p class="day">{{ day }}</p>
        <p class="month">{{ month }}</p>
    </div>
    <div class="more"></div>
</template>

<style scoped>

    p {
        position: relative;
        font-family: 'IBM Plex Sans JP', sans-serif;
        margin: 0;

        pointer-events: none;
    }

    div {
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