<script setup lang="ts">
    import router from '@src/router';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMounted, ref } from 'vue';
    import { useRoute } from 'vue-router';

    let dir = ref<string | null>(null);
    let dirContents = ref<string | null>(null);
    // defineProps(['currentDir']);
    const emits = defineEmits();

    function openDirectory(file: string) {
        const isDir = !file.includes('.');

        if (isDir) {
            invoke("open_dir", {dir: file}).then((data) => {
                dirContents.value = JSON.stringify(data);
                router.push({ name: 'directory', params: { dirName: JSON.stringify(data) } });
                setTimeout(() => {
                    window.location.reload();
                }, 1);
            }).catch((err) => {
                console.error(err);
            })
        }
    }

    onMounted(() => {
        if (useRoute().params.dirName === ':None') {
            return
        }
        dir.value = JSON.parse(useRoute().params.dirName as string);
        emits('dirDataSent', dir.value);
    });
</script>

<template>
    <ul class="w-screen p-4 bg-stone-900 flex flex-col gap-1">
        <li @dblclick="openDirectory(file)" v-for="file in dir" :key="file" 
            class="text-white w-88 h-8 bg-zinc-600 flex items-center gap-1 text-sm cursor-pointer hover:bg-zinc-500 p-2"
        >
            <img class="w-5 h-5" v-if="file.includes('.')" src="@src/assets/file.png" alt="">
            <img class="w-5 h-5" v-else src="@src/assets/directory.png" alt="">
            <p class="w-full overflow-hidden text-ellipsis text-nowrap inline-block whitespace-nowrap">
                {{ file }}
            </p>
    </li>
    </ul>
</template>
