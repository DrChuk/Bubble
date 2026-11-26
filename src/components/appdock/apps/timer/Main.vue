<script setup lang="ts">
    import { useTimer } from '../../../../composables/useTimer';

    const timer = useTimer(10);
</script>

<template>
    <div class="card">
        <div class="timer">
            <div class="square">
                <h1>{{ timer.returnString() }}</h1>
                <p>Timer</p>
            </div>
        </div>
    </div>
</template>

<style scoped>
    .card {
        display: flex;
        position: relative;

        justify-content: center;
        align-items: center;
        flex-direction: column;

        width: 95%;
        height: 95%;

        outline: 1px solid rgba(255, 255, 255, 0.1);
        border-radius: 15px;
        background-color: rgba(255, 255, 255, 0.05);
    }

    .card::after {
        content: "";
        position: absolute;

        top: 0;
        left: 0;
        bottom: 0;
        right: 0;

        border-radius: inherit;
        background: linear-gradient(
            45deg,
            #ffffff35 0%,
            #ffffff00 10%,
            #ffffff00 95%
        );
        padding: 1px;
        -webkit-mask: 
            linear-gradient(#fff 0 0) content-box,
            linear-gradient(#fff 0 0);
        -webkit-mask-composite: xor;
        mask-composite: exclude;
    }

    .timer {
        display: flex;
        position: relative;

        justify-content: center;
        align-items: center;
        flex-direction: column;
    }

    .square {
        display: flex;
        position: relative;

        justify-content: center;
        align-items: center;
        flex-direction: column;

        width: 192px;
        height: 192px;

        outline: 4px solid white;
        border-radius: 50px;
    }

    h1 {
        margin: 0;
        font-family: 'Quicksand', serif;
        font-size: 64px;
        font-weight: 900;

        color: white;
    }

    p {
        margin: 0;
        font-family: 'Quicksand', sans-serif;
        font-weight: 600;
        font-size: 18px;
        color: #BBB;
    }
</style>
