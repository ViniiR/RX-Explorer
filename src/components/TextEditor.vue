<script setup lang="ts">
    import { invoke } from '@tauri-apps/api';
    import { onMounted, ref } from 'vue';
    import { useRoute } from 'vue-router';

    const file = ref(JSON.parse(useRoute().params.fileName as string));
    const fileIndex = file.value.lastIndexOf("\\");
    const fileName = (file.value as string).slice(fileIndex + 1, -1);

    let fileInitialContent = ref<string | null>();

    onMounted(() => {
        invoke("read_file", {fileName: file.value})
        .then((data) => {
            fileInitialContent.value = data as string;
            console.log(fileInitialContent.value);
        })
    })

    function goRoot() {
        //TODO: go back to root discarding changes
    }

</script>

<template>
    <section class="min-h-screen w-full bg-glass">
        <nav class="bg-stone-800 w-full h-10 text-white flex items-center gap-1 px-2 py-1 justify-between">
            <h1 class="font-normal text-lg px-2">{{  fileName }}</h1>
            <div class="flex items-center w-24 h-full justify-around">
                <button @click="goRoot" class="h-10 w-10 hover:bg-zinc-600 p-2 rounded">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" style="fill: white;transform: ;msFilter:;"><path d="m21.983 13.821-1.851-10.18A1.998 1.998 0 0 0 18.165 2H5.835a2 2 0 0 0-1.968 1.643l-1.85 10.178.019.003c-.012.06-.036.114-.036.176v5c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2v-5c0-.063-.024-.116-.035-.176l.018-.003zM5.835 4h12.331l1.637 9H4.198l1.637-9zM4 19v-4h16l.002 4H4z"></path><path d="M17 16h2v2h-2zm-3 0h2v2h-2z"></path></svg>
                </button>
                <button class="h-10 w-10 hover:bg-zinc-600 p-2 rounded">
                    <img class="w-full h-full" src="@assets/save.png" alt="" srcset="">
                </button>
            </div>
            <!-- add more -->
        </nav>
        <textarea 
            v-model="fileInitialContent"
            name="editor" id="editor"
            class="glass w-full h-screen bg-transparent resize-none ps-5 pt-2 text-white"
        ></textarea>
        <footer class="text-white fixed bottom-0 bg-stone-800 p-2 w-full">
            shows current line or info idk 
        </footer>
    </section>
</template>

<style>
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