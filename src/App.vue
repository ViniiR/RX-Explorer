
<script setup lang="ts">
    import Nav from '@components/Nav.vue';
    import { onMounted } from 'vue';
    onMounted(() => {
        localStorage.setItem("currDir", "root")
    })
    window.addEventListener('beforeunload', function () {
        return 'bitch reloaded';
    });
    //ONLY ADD THIS LINE ON PRODUCTION, IT IS USEFUL FOR DEBUGGING PURPOSES ON DEVELOPMENT.
    document.addEventListener('contextmenu', event => event.preventDefault());
</script>

<template>
    <Nav></Nav>
</template>