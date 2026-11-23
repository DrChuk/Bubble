<script setup lang="ts">

</script>

<template>
    <div class="card">
        <span class="circle-timer"></span>
    </div>
</template>

<style scoped>
    .card {
        display: flex;
        position: relative;

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
</style>
