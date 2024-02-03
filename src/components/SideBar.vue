<script setup lang="ts">
    import router from '@src/router';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMounted, ref } from 'vue';

    const emit = defineEmits(['diskOpen', 'favoriteOpen']);
    let homeDir = ref<string | null>(null)

    async function openFavorite(file: string) {
        const homePath = (await invoke("get_favorites") as Array<string>)[0]

        let path = `${homePath}\\${file}`;
        if (file === 'home') {
            path = homePath;
        }

        const isDir = await invoke("is_dir", {file: path});

        if (isDir) {
            try {
                localStorage.setItem("currDir", path);
                emit('favoriteOpen')
                const openedDir: string[] = await invoke("open_dir", {dir: path});
                router.push(
                    {name: 'directory', params: { dirName: JSON.stringify(openedDir)}}
                );
            } catch (err) {
                console.error(err)
            }
        } else {
            router.push({ name: 'text', params: { filename: JSON.stringify(path) } });
        }
    }

    function openDisk() {
        localStorage.setItem("currDir", "C:\\");
        emit("diskOpen");
        invoke("open_disk", {diskName: 'C'})
            .then((data) => {
                router.push({ name: 'directory', params: { dirName: JSON.stringify(data) } });
            }).catch((err) => {
                console.error(err)
            })
    }

    async function setHomeDir() {
        try {
            const homePath: Array<string> = await invoke("get_favorites");
            if (homePath) {
                homeDir.value = homePath[0].substring(homePath[0].lastIndexOf('\\') + 1)
            }
        } catch (err) {
            console.error(err)
        }
    }

    onMounted(() => {
        setHomeDir()
    })

</script>

<template>
    <section class="bg-stone-800 col-span-1 w-full h-full row-span-1 text-white p-2">
        <ul class="flex flex-col gap-2 py-2">
            <li 
                class="gap-1 h-14 flex items-center p-2 w-full bg-stone-700 cursor-pointer hover:bg-stone-600 rounded"
                @click="openFavorite('home')"
            >
                <img class="w-5" src="@assets/ndHome.png" alt="">
                <strong>{{ homeDir }}</strong>
            </li>
            <li 
                class="gap-1 h-14 flex items-center p-2 w-full bg-stone-700 cursor-pointer hover:bg-stone-600 rounded"
                @click="openFavorite('Downloads')"
            >
                <img class="w-5" src="@assets/ndDownloads.png" alt="">
                <strong>Downloads</strong>
            </li>
            <li 
                class="gap-1 h-14 flex items-center p-2 w-full bg-stone-700 cursor-pointer hover:bg-stone-600 rounded"
                @click="openFavorite('Documents')"
            >
                <img class="w-5" src="@assets/ndDocuments.png" alt="">
                <strong>Documents</strong>
            </li>
            <li 
                class="gap-1 h-14 flex items-center p-2 w-full bg-stone-700 cursor-pointer hover:bg-stone-600 rounded"
                @click="openFavorite('Pictures')"
            >
                <img class="w-5" src="@assets/ndPictures.png" alt="">
                <strong>Pictures</strong>
            </li>
            <li 
                class="gap-1 h-14 flex items-center p-2 w-full bg-stone-700 cursor-pointer hover:bg-stone-600 rounded"
                @click="openDisk"
            >
                <img class="w-5" src="@assets/diskImage.png" alt="">
                <strong>C: Disk</strong>
            </li>
        </ul>
    </section>
</template>
