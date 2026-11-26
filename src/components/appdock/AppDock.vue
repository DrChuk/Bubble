<script setup lang="ts">
    import AppItem from './AppItem.vue';
    import {APPS} from "./../appdock/index.ts";
</script>

<template>
    <div class="apps">
        <AppItem 
            v-for="app in APPS"
            :id="app.id"
            :key="app.id"
            :title="app.title"
            :icon="app.icon"
        />
    </div>
</template>

<style scoped>
    .apps {
        display: flex;
        position: relative;
        width: auto;
        height: 48px;

        align-items: center;
        margin-bottom: 8px;
        gap: 8px;
        padding: 0 4px;

        background-color: rgba(255, 255, 255, 0.1);
        border-radius: 5px;
    }
</style>
