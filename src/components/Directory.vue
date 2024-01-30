<script setup lang="ts">
    import router from '@src/router';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMounted, ref } from 'vue';
    import { useRoute } from 'vue-router';
import FileImage from './FileImage.vue';

    let dir = ref<string | null>(null);
    let dirContents = ref<string | null>(null);
    let isEmpty = ref(false);

    async function openDirectory(file: string) {
        const isDir = await invoke("is_dir", {file});
        try {
            if (isDir) {
                localStorage.setItem("currDir", file);
                console.log(localStorage.getItem("currDir"));
                const dirFiles = await invoke("open_dir", {dir: file})
                const data = JSON.stringify(dirFiles);
                dirContents.value = data
                router.push({name: 'directory', params: { dirName: data}})
                setTimeout(() => {
                    window.location.reload()
                }, 100);
            } else {
                router.push({ name: 'text', params: { fileName: JSON.stringify(file) } });
            }
        } catch (err) {
            console.error(err)
        }
    }

    function isDescendant(parent: ParentNode, child: HTMLElement): boolean {
        let node = child.parentNode;
        while (node != null) {
            if (node === parent) {
                return true;
            }
            node = node.parentNode;
        }
        return false;
    }

    async function deleteFile(file: string) {
        try {
            invoke("delete_file", {filePath: file})
            console.log(localStorage.getItem("currDir"));
            openDirectory(localStorage.getItem("currDir")!)
        } catch (err) {
            console.error(err)
        }
    }

    async function readToClipboard(fileName: string) {
        const content: string = await invoke("read_file", {fileName})
        navigator.clipboard.writeText(content)
    }
    
    function showContext(event: Event) {
        event.preventDefault()
        let target = event.target as HTMLElement;
        if (target.tagName !== 'LI') {
            target = target.parentNode as HTMLLIElement;
        }
        const context = target.querySelector('section.context-menu') as HTMLDivElement;
        context.style.display = 'flex'
        document.addEventListener("click", function (event) {
            const clickTarget = event.target as HTMLElement;
            if (!isDescendant(context.parentNode!, clickTarget)) {
                context.style.display = "none"
            }
        });
    }

    onMounted(() => {
        if (useRoute().params.dirName === ':None') {
            return
        }
        dir.value = JSON.parse(useRoute().params.dirName as string);
        if (localStorage.getItem("currDir") === "root" && dir.value?.length! > 0) {
            const subString = dir.value![0].substring(0, dir.value![0].lastIndexOf('\\'));
            localStorage.setItem("currDir", subString)
        }
        isEmpty.value = dir.value?.length! <= 0;
    });

</script>

<template>
    <ul class="pt-14 pb-14 w-screen min-h-screen p-4 bg-stone-900 flex flex-col gap-1 ">   
        <li v-if="isEmpty" class="pointer-events-none rounded text-white text-center bg-zinc-800 p-2">
            Directory empty or you do not have permissions to access this directory
        </li>
        <li @dblclick="openDirectory(file)" v-for="file in dir" :key="file" 
            class="text-white w-88 h-8 flex items-center gap-1 text-sm cursor-pointer hover:bg-zinc-500 p-2 relative"
            @auxclick="showContext"
        >
            <FileImage :file="file"></FileImage>
            <p class="w-full overflow-hidden text-ellipsis text-nowrap inline-block whitespace-nowrap">
                {{ file }}
            </p>
            <section 
                class="context-menu rounded bg-zinc-800 w-52 h-52 absolute left-1/2 top-0 hidden p-2 z-10 shadow-md"
            >
                <ul class="h-full w-full flex flex-col gap-2">
                    <li 
                        class="hover:bg-zinc-500 bg-zinc-700 p-2 rounded w-full h-10 flex gap-1"
                    >
                        <img class="p-1 h-full" src="@assets/copyFile.png" alt="" srcset="">
                        <strong @click="readToClipboard(file)">Copy as Text</strong>
                    </li>
                    <li 
                        class="hover:bg-zinc-500 bg-zinc-700 p-2 rounded w-full h-10 flex gap-1"
                    >
                        <img class="p-1 h-full" src="@assets/trash.png" alt="" srcset="">
                        <strong @click="deleteFile(file)">Delete</strong>
                    </li>
                </ul>
            </section>
        </li>
    </ul>
</template>
