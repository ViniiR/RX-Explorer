<script setup lang="ts">
    import router from '@src/router';
    import { invoke } from '@tauri-apps/api';
    import { onMounted, ref } from 'vue';
    import { useRoute } from 'vue-router';

    const file = ref(JSON.parse(useRoute().params.fileName as string));
    const fileIndex = file.value.lastIndexOf("\\");
    const fileName = (file.value as string).slice(fileIndex + 1);
    const textAreaRef = ref<HTMLTextAreaElement | null>(null)
    let displayText = ref<HTMLDivElement | null>(null)

    let fileInitialContent = ref<string | null>();

    onMounted(() => {
        window.scrollTo(0, 0)
        invoke("read_file", {fileName: file.value})
        .then((data) => {
            fileInitialContent.value = data as string;
        })
        .catch((err) => {
            fileInitialContent.value = err;
        })
    })
    
    function goBack() {
        const pwd = localStorage.getItem("currDir");
        console.log(pwd);
        if (pwd === 'root') return;
        invoke("open_dir", {dir: pwd}).then((data) => {
                router.push(
                    { name: 'directory', params: { dirName: JSON.stringify(data) } }
                );
            }).catch((err) => {
                console.error(err);
            })
    }

    function save() {
        invoke("save_file", {file: file.value, text: fileInitialContent.value})
    }

    function goRoot() {
        localStorage.setItem("currDir", "root")
        router.push({name: "disks"});
    }

    function showDisplayText(text: string) {
        if (!displayText.value) return;
        displayText.value.innerText = text;
        displayText.value.style.display = 'flex';
        setTimeout(() => {
            displayText.value!.style.display = "none"
        }, 500);
    }
    
    function textAreaKeyHandler(event: KeyboardEvent) {
        if (event.key === "Tab") {
            event.preventDefault()
            const target = event.target as HTMLTextAreaElement
            target.setRangeText(
                '\t',
                target.selectionStart,
                target.selectionStart,
                'end'
            );
        } else if (event.key === "s" && event.ctrlKey) {
            event.preventDefault()
            showDisplayText("File Saved")
            save();
        }
    };
</script>

<template>
    <section class="min-h-screen w-full bg-glass">
        <nav class="bg-stone-800 w-full h-10 text-white flex items-center gap-1 px-2 py-1 justify-between">
            <h1 class="font-normal text-lg px-2">{{ fileName }}</h1>
            <div class="flex items-center w-32  h-full justify-around">
                <button @click="goRoot" class="h-10 w-10 hover:bg-zinc-600 p-2 rounded">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" style="fill: white;transform: ;msFilter:;"><path d="m21.983 13.821-1.851-10.18A1.998 1.998 0 0 0 18.165 2H5.835a2 2 0 0 0-1.968 1.643l-1.85 10.178.019.003c-.012.06-.036.114-.036.176v5c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2v-5c0-.063-.024-.116-.035-.176l.018-.003zM5.835 4h12.331l1.637 9H4.198l1.637-9zM4 19v-4h16l.002 4H4z"></path><path d="M17 16h2v2h-2zm-3 0h2v2h-2z"></path></svg>
                </button>
                <button 
                    class="h-10 w-10 hover:bg-zinc-600 p-2 rounded"
                    @click="save"
                >
                    <img class="w-full h-full" src="@assets/save.png" alt="" srcset="">
                </button>
                <button @click="goBack" class="text-white w-10 flex items-center justify-center h-10 hover:bg-zinc-600 rounded">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" style="fill: white;transform: ;msFilter:;"><path d="M11 8.414V18h2V8.414l4.293 4.293 1.414-1.414L12 4.586l-6.707 6.707 1.414 1.414z"></path></svg>
                </button>
            </div>
            <!-- add more -->
        </nav>
        <div
            ref="displayText"
            class="
                w-32 h-14 bg-stone-600 text-white rounded fixed top-10 z-10 left-1/2 -translate-x-1/2
                p-2 items-center justify-center hidden 
            "
        >
        ...
        </div>
        <textarea 
            ref="textAreaRef"
            @keydown="textAreaKeyHandler"
            v-model="fileInitialContent"
            spellcheck="false"
            name="editor" id="editor"
            class="glass text-area-font text-sm w-full h-screen bg-transparent resize-none ps-5 pt-2 text-white pb-1"
        ></textarea>
        <!-- <footer class="text-white fixed bottom-0 bg-stone-800 p-2 w-full">
            shows current line or info idk 
        </footer> -->
    </section>
</template>

<style>
    @import url(https://fonts.googleapis.com/css?family=Roboto+Mono:100,200,300,regular,500,600,700,100italic,200italic,300italic,italic,500italic,600italic,700italic);
    
    .text-area-font {
        font-family: "Roboto Mono";
    }

    .glass {
        background: 
            linear-gradient(135deg,
                #1e1e1ecc, 
                #1e1e1ecc
            );
        backdrop-filter: blur(20px);
        -webkit-backdrop-filter: blur(20px);
    }

    .bg-glass {
        background-image: url("@assets/bg.jpg");
        background-size: cover;
    }
</style>