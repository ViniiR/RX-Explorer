<script setup lang="ts">
    import router from '@src/router';
    import { invoke } from '@tauri-apps/api';
    import { onUpdated, ref } from 'vue';
    import { useRouter } from 'vue-router';
import SideBar from './SideBar.vue';

    let pwd = ref<string | null>(localStorage.getItem("currDir"));
    const route = useRouter();
    let textFormRef = ref<HTMLFormElement | null>(null);
    let directoryFormRef = ref<HTMLFormElement | null>(null);
    let currentTime = ref<string | null>(null);
    let currentDate = ref<string | null>(null)
    let displayText = ref<HTMLDivElement | null>(null)

    const MONTHS_OF_THE_YEAR = [
        "Jan.",
        "Feb.",
        "Mar.",
        "Apr.",
        "May.",
        "Jun.",
        "Jul.",
        "Aug.",
        "Sep.",
        "Oct.",
        "Nov.",
        "Dec.",
    ]

    function setLeadingZero(string: string): string {
        if (string.length === 1) {
            return string.padStart(2, '0');
        }
        return string;
    }

    function setIndicator(number: number) {
        const string = number.toString();
        switch (string.at(-1)) {
            case '1':
                return string + "st";
            case '2':
                return string + "nd";
            case '3':
                return string + "rd";
            default:
                return string + "th";
        }
    }

    setInterval(() => {
        const date = new Date()
        currentTime.value = 
        `
            ${
                date.getHours()
            } : ${
                setLeadingZero(date.getMinutes().toString())
            } : ${
                setLeadingZero(date.getSeconds().toString())
            }
        `;
        currentDate.value = 
        `
            ${
                setIndicator(date.getDate())
            } / ${
                MONTHS_OF_THE_YEAR[date.getMonth()]
            } / ${
                date.getFullYear()
            }
        `;
    }, 1000);

    function goBack() {
        if (route.currentRoute.value.path === "/disks") return;
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
        pwd.value = "root"
        router.push({name: "disks"});
    }

    function updateRef() {
        pwd.value = localStorage.getItem("currDir");
    }

    function updateFromFav() {
        updateRef()
        if (router.currentRoute.value.path.includes('/dir')) {
            setTimeout(() => {
                window.location.reload()
            }, 10);
        }
    }

    function searchFile(data: Event) {
        data.preventDefault()
        const input = data.target as HTMLInputElement[] | null;
        const file = input![0].value as string;
        if (!file) {
            return
        }
        let path = pwd.value
        if (!path || path === 'root') {
            path = "C:\\"
        }
        turnDisplayText("Searching...", true);
        invoke("search_file", {file, path})
        .then((res) => {
            turnDisplayText('', false)
            let updatedRes = res as Array<string>;
            if (updatedRes.length < 1) {
                updatedRes = ["No files found"]
            }
            router.push({name: 'directory', params: {dirName: JSON.stringify(updatedRes)}})
            setTimeout(() => {
                window.location.reload();
            }, 1);
        }).catch(() => {
            showDisplayText("No files found")
        })
    }

    function showCreateFile() {
        if (directoryFormRef.value) {
            directoryFormRef.value.style.display = "none"
        }
        document.addEventListener("click", function (event) {
            const element = event.target as HTMLElement;
            if (element.parentNode!.contains(textFormRef.value)) {
                if (textFormRef.value && textFormRef) {
                    textFormRef.value.style.display = "grid"
                }
            } else {
                if (textFormRef.value && textFormRef) {
                    textFormRef.value.style.display = "none"
                }
            }
        });
    }
    
    function showCreateDir() {
        if (textFormRef.value) {
            textFormRef.value.style.display = "none"
        }
        document.addEventListener("click", function (event) {
            const element = event.target as HTMLElement;
            if (element.parentNode!.contains(directoryFormRef.value)) {
            if (directoryFormRef.value && directoryFormRef) {
                    directoryFormRef.value.style.display = "grid"
                }
            } else {
                if (directoryFormRef.value && directoryFormRef) {
                    directoryFormRef.value.style.display = "none"
                }
            }
        });
    }
    
    function createFile(event: Event) {
        if (pwd.value === "root") return;
        event.preventDefault()
        const target = event.target as HTMLFormElement;
        const formData = new FormData(target);
        const name = formData.get("fileName") as string
        const extension = formData.get("fileExtension") as string
        if (name.includes(".") || extension[0] !== '.' || !extension.includes("."))return;

        const file = `${name}${extension}`;
        invoke('create_file', {fileName: file, fileLocation: pwd.value})
            .then((data) => {
                router.push({ name: 'text', params: { fileName: JSON.stringify(data) } });
            })
    }

    async function mkdir(event: Event) {
        event.preventDefault();
        if (pwd.value === "root") return;
        const target = event.target as HTMLFormElement;
        const formData = new FormData(target);
        const dirName = formData.get("dirName");
        try {
            await invoke('create_directory', {dirName, dirLocation: pwd.value});
            const data = await invoke("open_dir",{dir: pwd.value});
            router.push(
                {name: "directory", params: {dirName: JSON.stringify(data as string)}}
            );
            setTimeout(() => {
                window.location.reload()
            }, 1);
        } catch (err) {
            console.error(err);
        }
    }

    onUpdated(() => {
        if (route.currentRoute.value.path === "/disks") {
            //TODO: this is trouble, try reviewing it later
            updateRef()
            return;
        }
    })
    
    function turnDisplayText(text: string, bool: boolean) {
        if (!displayText.value) return;
        if (bool) {
            displayText.value.innerText = text;
            displayText.value.style.display = 'flex';
        } else {
            displayText.value!.style.display = "none"
        }
    }
    
    function showDisplayText(text: string) {
        if (!displayText.value) return;
        displayText.value.innerText = text;
        displayText.value.style.display = 'flex';
        setTimeout(() => {
            displayText.value!.style.display = "none"
        }, 800);
    }
    
</script>

<template>
    <div class="w-full">
        <div
            ref="displayText"
            class="
                w-32 h-14 bg-stone-600 text-white rounded fixed top-10 z-10 left-1/2 -translate-x-1/2
                p-2 items-center justify-center hidden 
            "
        >
        ...
        </div>
        <nav v-if="!(route.currentRoute.value.path as string).includes('/text')"
            v-show="!(route.currentRoute.value.path as string).includes('/text')"
            class="flex justify-between px-1 fixed w-full bg-inherit h-10 bg-zinc-800 z-20"
        >
            <button @click="goDisk" class="text-white w-11 flex items-center justify-center hover:bg-zinc-600 rounded">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" style="fill: white;transform: ;msFilter:;"><path d="m21.983 13.821-1.851-10.18A1.998 1.998 0 0 0 18.165 2H5.835a2 2 0 0 0-1.968 1.643l-1.85 10.178.019.003c-.012.06-.036.114-.036.176v5c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2v-5c0-.063-.024-.116-.035-.176l.018-.003zM5.835 4h12.331l1.637 9H4.198l1.637-9zM4 19v-4h16l.002 4H4z"></path><path d="M17 16h2v2h-2zm-3 0h2v2h-2z"></path></svg>
            </button>
            <section class="flex w-full justify-end gap-1 items-center ps-1 pe-2">
                <form class=" w-full items-center flex h-full gap-1" @submit="searchFile">
                    <input class="w-full h-5/6 bg-white rounded p-2 flex items-center" type="text" name="" id="" :placeholder="pwd!">
                    <input class="cursor-pointer w-20 bg-blue-700 h-5/6 text-white rounded flex items-center justify-center hover:bg-white hover:text-blue-700 transition-all linear delay-100" type="submit" value="search">
                </form>
                <button class="w-12 p-2 h-full rounded hover:bg-zinc-600"
                    @click="showCreateFile"
                >
                    <form
                        class="fixed top-10 w-36 h-max bg-zinc-700 rounded text-white right-0 grid-rows-2 grid-cols-2 hidden"
                        ref="textFormRef"
                        @submit="createFile"
                    >
                        <label class="col-span-2 h-10 rounded flex items-center justify-center bg-zinc-600">New File</label>
                        <input
                            class="bg-transparent p-2"
                        required type="text" name="fileName" id="fileName"
                        placeholder="main" autocomplete="false">
                        <input
                            class="bg-transparent p-2"
                        type="text" name="fileExtension" id="fileExtension" placeholder=".rs" autocomplete="false">
                        <input
                            class="rounded p-2 text-center col-span-2 hover:bg-blue-500 cursor-pointer"
                        type="submit" name="submitFile" id="submitFile" value="create">
                    </form>
                    <img class="w-full" src="@assets/newFile.png" alt="" srcset="">
                </button>
                <button class="w-12 p-2 h-full rounded hover:bg-zinc-600 "
                    @click="showCreateDir"
                >
                    <form
                    class="fixed top-10 w-36 h-max bg-zinc-700 rounded text-white right-0 grid-rows-2 grid-cols-2 hidden"
                        ref="directoryFormRef"
                        @submit="mkdir"
                    >
                        <label class="col-span-2 h-10 rounded flex items-center justify-center bg-zinc-600">New Directory</label>
                        <input
                            class="bg-transparent p-2 w-full col-span-2"
                        required type="text" name="dirName" id="dirName" placeholder="directory" autocomplete="false">
                        <input
                            class="p-2 text-center col-span-2 hover:bg-blue-500 cursor-pointer rounded"
                        type="submit" name="submitDir" id="submitDir" value="create">
                    </form>
                    <img class="w-full" src="@assets/newDir.png" alt="" srcset="">
                </button>
                <button @click="goBack" class="text-white w-12 flex items-center justify-center h-full hover:bg-zinc-600 rounded">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" style="fill: white;transform: ;msFilter:;"><path d="M11 8.414V18h2V8.414l4.293 4.293 1.414-1.414L12 4.586l-6.707 6.707 1.414 1.414z"></path></svg>
                </button>
            </section>
        </nav>
        <div class="grid sidebar-wrapper grid-rows-1 custom-height-wrapper w-screen"
            :style="{paddingTop: (route.currentRoute.value.path as string).includes('/text') ? '0px' : '40px'}"
        >
            <SideBar @diskOpen="updateFromFav" @favoriteOpen="updateFromFav"></SideBar>
            <RouterView class="col-span-1 row-span-1" @diskOpen="updateRef" @reqForUpdate="updateRef"></RouterView>
        </div>
        <footer class="bottom-0 fixed h-10 w-full bg-zinc-800 text-white p-2 flex justify-between">
            <p>
                {{ currentDate }}
            </p>
            <p>
                {{ currentTime }}
            </p>
        </footer>
    </div>
</template>

<style scoped>
    .sidebar-wrapper {
        grid-template-columns: 200px 1fr;
    }
    .custom-height-wrapper {
        height: calc(100vh - 40px);
    }
</style>