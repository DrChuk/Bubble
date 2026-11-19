<script setup lang="ts">
    import AppDock from "./../appdock/AppDock.vue";

</script>

<template>
    <div class="container">
        <AppDock />
    </div>
</template>

<style scoped>
    .container {
        display: flex;
        position: relative;

        width: 480px;
        height: 370px;

        background: linear-gradient(
            45deg,
            #222,
            #000
        );
    }
</style>
