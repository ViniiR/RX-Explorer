
<script setup lang="ts">
    import Nav from '@components/Nav.vue';
    import { onMounted } from 'vue';
    onMounted(() => {
        localStorage.setItem("currDir", "root")
    })
</script>

<template>
    <Nav></Nav>
</template>