<script setup lang="ts">
    import router from '@src/router';
    import { invoke } from '@tauri-apps/api';
    import { onMounted, ref } from 'vue';

    import downloadsImage from '@assets/downloads.png'
    import documentsImage from '@assets/documents.png'
    import picturesImage from '@assets/pictures.png'
    import homeImage from '@assets/home.png'

    const emit = defineEmits(['favoriteOpen'])
    
    let downloadsPath = ref<string | null>(null);
    let documentsPath = ref<string | null>(null);
    let picturesPath = ref<string | null>(null);
    let homePath = ref<string | null>(null);

    async function setFavorites() {
        const favorites: string[] = await invoke("get_favorites");
        favorites.forEach((element) => {
            const elementName = element.substring(element.lastIndexOf('\\') + 1).toLowerCase();
            switch (elementName) {
                case 'downloads':
                    downloadsPath.value = element
                    return;
                case 'documents':
                    documentsPath.value = element
                    return;
                case 'pictures':
                    picturesPath.value = element
                    return;
                default:
                    homePath.value = element
                    return;
            }
        });
    }

    onMounted(() => {
        setFavorites();
    })

    async function openDirectory(file: string) {
        const isDir = await invoke("is_dir", {file});

        if (isDir) {
            try {
                localStorage.setItem("currDir", file);
                emit('favoriteOpen')
                const openedDir: string[] = await invoke("open_dir", {dir: file});
                router.push(
                    {name: 'directory', params: { dirName: JSON.stringify(openedDir)}}
                );
            } catch (err) {
                console.error(err)
            }
        } else {
            router.push({ name: 'text', params: { filename: JSON.stringify(file) } });
        }
    }
    
</script>

<template>
    <ul class="grid grid-rows-2 grid-cols-2 gap-1 h-max w-full col-span-2 row-span-2">
        <li class="w-full bg-zinc-800 cursor-pointer p-4 h-24 text-xs rounded flex gap-5 hover:bg-stone-800 col-span-1"
            @dblclick="openDirectory(homePath!)"
        >
            <div class="w-16 flex items-center h-full ">
                <img class="w-full" :src="homeImage" alt="SSD image">
            </div>
            <strong class="text-lg">{{ homePath?.substring(homePath.lastIndexOf('\\') + 1) }}</strong>
        </li>
        <li class="w-full bg-zinc-800 cursor-pointer p-4 h-24 text-xs rounded flex gap-5 hover:bg-stone-800"
            @dblclick="openDirectory(downloadsPath!)"
        >
            <div class="w-16 flex items-center h-full ">
                <img class="w-full" :src="downloadsImage" alt="SSD image">
            </div>
            <strong class="text-lg">Downloads</strong>
        </li>
        <li class="w-full bg-zinc-800 cursor-pointer p-4 h-24 text-xs rounded flex gap-5 hover:bg-stone-800"
            @dblclick="openDirectory(documentsPath!)"
        >
            <div class="w-16 flex items-center h-full ">
                <img class="w-full" :src="documentsImage" alt="SSD image">
            </div>
            <strong class="text-lg">Documents</strong>
        </li>
        <li class="w-full bg-zinc-800 cursor-pointer p-4 h-24 text-xs rounded flex gap-5 hover:bg-stone-800"
            @dblclick="openDirectory(picturesPath!)"
        >
            <div class="w-16 flex items-center h-full ">
                <img class="w-full" :src="picturesImage" alt="SSD image">
            </div>
            <strong class="text-lg">Pictures</strong>
        </li>
    </ul>
</template>