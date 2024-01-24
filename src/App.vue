
<script setup lang="ts">
    import Nav from '@components/Nav.vue';
    import { onMounted } from 'vue';
    onMounted(() => {
        localStorage.setItem("currDir", "root")
    })
    window.addEventListener('beforeunload', function () {
        return 'bitch reloaded';
    });
</script>

<template>
    <Nav></Nav>
</template>