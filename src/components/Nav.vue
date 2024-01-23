<script setup lang="ts">
    import router from '@src/router';
    import { invoke } from '@tauri-apps/api';
    import { ref } from 'vue';

    let currentDir = ref<string | null>(null);
    let dirContents = ref<string | null>(null);
    let currentDirPath = ref<string | null>(null);

    function goBack() {
        if (currentDirPath.value === "C:\\") {
            router.push({path: "/disks"})
            currentDirPath.value = "\\root";
            return;
        } else if (currentDirPath.value === "\\root") {
            return
        }
        invoke("open_dir", {dir: currentDir.value}).then((data) => {
            dirContents.value = JSON.stringify(data);
            router.push({ name: 'directory', params: { dirName: JSON.stringify(data) } });
            setTimeout(() => {
                window.location.reload();
            }, 1);
        }).catch((err) => {
            console.error(err);
        })
    }

    function goDisk() {
        router.push({name: "disks"});
        currentDirPath.value = "\\root";
    }

    function getCurrentDir(data: Array<String>) {
        if (data.length < 1) return;
        const index = data[0].lastIndexOf('\\');
        let slicedString = data[0].substring(0, index);
        if (slicedString === "C:") {
            slicedString = "C:\\";
        }
        const lastDirIndex = slicedString.lastIndexOf('\\');
        const lastDir = slicedString.substring(0, lastDirIndex);
        currentDirPath.value = slicedString;
        currentDir.value = lastDir;
    }
    
</script>

<template>
    <nav class="flex justify-between px-1 fixed w-full bg-inherit h-10">
        <button @click="goDisk" class="text-white w-10 flex items-center justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" style="fill: white;transform: ;msFilter:;"><path d="m21.983 13.821-1.851-10.18A1.998 1.998 0 0 0 18.165 2H5.835a2 2 0 0 0-1.968 1.643l-1.85 10.178.019.003c-.012.06-.036.114-.036.176v5c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2v-5c0-.063-.024-.116-.035-.176l.018-.003zM5.835 4h12.331l1.637 9H4.198l1.637-9zM4 19v-4h16l.002 4H4z"></path><path d="M17 16h2v2h-2zm-3 0h2v2h-2z"></path></svg>
        </button>
        <section class="flex w-full justify-end gap-1 items-center ps-1 pe-2">
            <section class="bg-white w-full h-4/6 items-center flex rounded p-2">
                {{ currentDirPath }}
            </section>
            <button @click="goBack" class="text-white w-6">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" style="fill: white;transform: ;msFilter:;"><path d="M12.707 17.293 8.414 13H18v-2H8.414l4.293-4.293-1.414-1.414L4.586 12l6.707 6.707z"></path></svg>
            </button>
            <RouterLink class="text-white w-6" to="/Dir:None">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" style="fill: white;transform: ;msFilter:;"><path d="m11.293 17.293 1.414 1.414L19.414 12l-6.707-6.707-1.414 1.414L15.586 11H6v2h9.586z"></path></svg>
            </RouterLink>
        </section>
    </nav>
    <RouterView @dirDataSent="getCurrentDir" class="pt-14"></RouterView>
</template>