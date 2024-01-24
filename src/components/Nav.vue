<script setup lang="ts">
    import router from '@src/router';
    import { invoke } from '@tauri-apps/api';
    import { ref } from 'vue';
    import { useRouter } from 'vue-router';

    let pwd = ref<string | null>(localStorage.getItem("currDir"))
    const route = useRouter();

    function goBack() {
        const prevDirIndex = pwd.value?.lastIndexOf("\\");
        let prevDir = pwd.value?.substring(0, prevDirIndex!);
        if (prevDir === "C:") {
            prevDir = "C:\\"
        }
        localStorage.setItem("currDir", prevDir!);
        pwd.value = prevDir!
        invoke("open_dir", {dir: prevDir}).then((data) => {
                router.push({ name: 'directory', params: { dirName: JSON.stringify(data) } });
                setTimeout(() => {
                    window.location.reload();
                }, 1);
            }).catch((err) => {
                console.error(err);
            })
    }

    function goDisk() {
        localStorage.setItem("currDir", "root")
        router.push({name: "disks"});
        pwd.value = "root"
    }

    function updateRef() {
        pwd.value = localStorage.getItem("currDir");
    }

    function searchFile(data: Event) {
        data.preventDefault()
        const input = data.target as HTMLInputElement[] | null;
        const file = input![0].value as string;
        if (!file) {
            return
        }
        invoke("search_file", {file, path: "C:\\Users\\Vinii"}).then((res) => {
            let updatedRes = res as Array<string>;
            if (updatedRes.length < 1) {
                updatedRes = ["No files found, change this output later pls im at NavVue line 43"]
            }
            router.push({name: 'directory', params: {dirName: JSON.stringify(updatedRes)}})
            setTimeout(() => {
                window.location.reload();
            }, 1);
        })
    }
    
</script>

<template>
    <nav v-if="!(route.currentRoute.value.path as string).includes('/text')" class="flex justify-between px-1 fixed w-full bg-inherit h-10 bg-zinc-800">
        <button @click="goDisk" class="text-white w-10 flex items-center justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" style="fill: white;transform: ;msFilter:;"><path d="m21.983 13.821-1.851-10.18A1.998 1.998 0 0 0 18.165 2H5.835a2 2 0 0 0-1.968 1.643l-1.85 10.178.019.003c-.012.06-.036.114-.036.176v5c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2v-5c0-.063-.024-.116-.035-.176l.018-.003zM5.835 4h12.331l1.637 9H4.198l1.637-9zM4 19v-4h16l.002 4H4z"></path><path d="M17 16h2v2h-2zm-3 0h2v2h-2z"></path></svg>
        </button>
        <section class="flex w-full justify-end gap-1 items-center ps-1 pe-2">
            <form class=" w-full items-center flex h-full gap-1" @submit="searchFile">
                <input class="w-full h-5/6 bg-white rounded p-2 flex items-center" type="text" name="" id="" :placeholder="pwd!">
                <input class="cursor-pointer w-20 bg-blue-700 h-5/6 text-white rounded flex items-center justify-center hover:bg-blue-600" type="submit" value="search">
            </form>
            <button @click="goBack" class="text-white w-6">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" style="fill: white;transform: ;msFilter:;"><path d="M11 8.414V18h2V8.414l4.293 4.293 1.414-1.414L12 4.586l-6.707 6.707 1.414 1.414z"></path></svg>
            </button>
        </section>
    </nav>
    <RouterView class="" @diskOpen="updateRef"></RouterView>
</template>
