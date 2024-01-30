<script setup lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMounted, ref } from 'vue';
    import { useRouter } from 'vue-router';
    import TaskManager from './TaskManager.vue';

    type DirectoriesObjects = {
        path: string;
        imagePath: string;
        name: string;
    }

    let Disk = ref<Disk | null>(null);
    let CurrentDirectoryContents = ref<string | null>(null);
    const router = useRouter();
    const emit = defineEmits(['diskOpen'])
    let homeDir = ref<string | null>(null);
    let favoriteDirectories = ref<DirectoriesObjects[] | null>(null)

    function startTask() {
        return invoke("display_disks");
    }

    function openDisk() {
        localStorage.setItem("currDir", "C:\\");
        emit("diskOpen");
        invoke("open_disk", {diskName: Disk.value?.name})
            .then((data) => {
                CurrentDirectoryContents.value = JSON.stringify(data);
                router.push({ name: 'directory', params: { dirName: JSON.stringify(data) } });
            })
    }

    function bytesToGiB(bytes: number): number {
        return bytes / Math.pow(1024, 3);
    }
    
    startTask().then((data) => {
        Disk.value = data as Disk;
    });

    onMounted(async () => {
        const data: string[] = await invoke("get_favorites")
        homeDir.value = data[0];
        const directories = [...data];
        directories.shift()
        const obj: DirectoriesObjects[] = directories.map((item) => ({
            name: item.substring(item.lastIndexOf('\\') + 1),
            path: item,
            imagePath: 
                `src/assets/${item.substring(item.lastIndexOf('\\') + 1).toLocaleLowerCase()}.png`
        }))
        favoriteDirectories.value = obj;
    })

    async function openDirectory(file: string) {
        const isDir = await invoke("is_dir", {file});

        if (isDir) {
            try {
                localStorage.setItem("currDir", file);
                emit("diskOpen");
                const openedDir: string[] = await invoke("open_dir", {dir: file});
                router.push(
                    {name: 'directory', params: { dirName: JSON.stringify(openedDir)}}
                );
            } catch (err) {
                console.error(err)
            }
        } else {
            router.push({ name: 'text', params: { fileName: JSON.stringify(file) } });
        }
    }
    
</script>
<template>
    <ul class="pt-14 w-full min-h-28 px-2 text-neutral-50 grid grid-cols-2 grid-rows-7 gap-1 select-none">
        <li @dblclick="openDisk" class="w-full bg-zinc-800 cursor-pointer p-4 h-24 text-xs rounded flex gap-5 hover:bg-stone-800 col-span-2">
            <div class="w-16 flex items-center h-full ">
                <img class="w-full" v-if="Disk?.diskType === 'SSD'" src="@src/assets/ssd.png" alt="SSD image">
                <img class="w-full" v-else src="@src/assets/hdd.png" alt="HDD image">
            </div>
            <div class="w-full flex flex-col justify-between">
                <div class="flex-row flex w-full items-center gap-3 justify-between">
                    <strong class="text-lg">Disk {{ Disk?.name }}:</strong>
                    <p class="text-sm font-medium">{{ Disk?.fileSystem }}</p>
                </div>
                <div class="flex gap-1 items-end">
                    <p class="text-sm">
                        {{ 
                            bytesToGiB((Number(Disk?.totalCapacity) - Number(Disk?.availableCapacity))).toFixed(2)
                        }}<span> GiB </span>
                    </p>
                    <p class="text-sm">/</p>
                    <p class="text-sm">
                        {{
                            bytesToGiB(Number(Disk?.totalCapacity)).toFixed(2) 
                        }}<span> GiB</span>
                    </p>
                    <p class="text-sm ml-auto">
                        {{ Disk?.usagePercentage }}%
                    </p>
                </div>
                <div class="w-full bg-white h-6 border-2 border-white">
                    <section class="bg-blue-500 h-full" :style="{width: `${Disk?.usagePercentage}%`}">
                        
                    </section>
                </div>
            </div>
        </li>

        <!-- C IS UP DIR DOWN -->

        <li class="w-full bg-zinc-800 cursor-pointer p-4 h-24 text-xs rounded flex gap-5 hover:bg-stone-800 col-span-1" 
            @dblclick="openDirectory(homeDir!)">
            <div class="w-16 flex items-center h-full ">
                <img class="w-full" src="@src/assets/home.png" alt="SSD image">
            </div>
            <strong class="text-lg">Vinii</strong>
        </li>
        <li class="w-full bg-zinc-800 cursor-pointer p-4 h-24 text-xs rounded flex gap-5 hover:bg-stone-800"
            v-for="obj in favoriteDirectories"
            @dblclick="openDirectory(obj.path)"
        >
            <div class="w-16 flex items-center h-full ">
                <img class="w-full" :src="obj.imagePath" alt="SSD image">
            </div>
            <strong class="text-lg">{{ obj.name }}</strong>
        </li>
        <li class="col-fix col-span-2 bg-zinc-800 rounded w-full">
            <TaskManager></TaskManager>
        </li>
    </ul>
</template>

<style>
    .col-fix {
        grid-row: 4 / 9;
        display: block;
    }
</style>