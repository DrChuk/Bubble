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
            :component="app.compoonent"
        />
    </div>
</template>

<style scoped>
    .apps {
        display: flex;
        position: relative;
        width: 90%;
        height: 36px;

        align-self: end;
        margin-bottom: 8px;

        background-color: lightgray;
        border-radius: 15px;
    }
</style>
