<script setup lang="ts">
    import AppItem from './AppItem.vue';
    import {APPS} from "./../appdock/index.ts";
</script>

<template>
    <div class="apps">
        <AppItem 
            v-for="app in APPS"
            :id="app.id"
            :title="app.title"
            :icon="app.icon"
            :component="app.component"
        />
    </div>
</template>

<style scoped>
    .apps {
        display: flex;
        position: relative;
        width: auto;
        height: 40px;

        align-self: end;
        align-items: center;
        margin-bottom: 8px;
        gap: 8px;
        padding: 0 2px;

        background: rgba(255, 255, 255, 0.1);
        border: 1px solid rgba(255, 255, 255, 0.3);
        backdrop-filter: blur(20px);
        border-radius: 5px;
    }
</style>
