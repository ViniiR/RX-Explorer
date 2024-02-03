<script setup lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { ref } from 'vue';
    import { useRouter } from 'vue-router';
    import TaskManager from './TaskManager.vue';
    import FavoriteDirectories from './FavoriteDirectories.vue';

    let Disk = ref<Disk | null>(null);
    let CurrentDirectoryContents = ref<string | null>(null);
    const router = useRouter();
    const emit = defineEmits(['diskOpen', 'reqForUpdate'])

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

    function emitFavorite() {
        emit("reqForUpdate")
    }

    function bytesToGiB(bytes: number): number {
        return bytes / Math.pow(1024, 3);
    }
    
    startTask().then((data) => {
        Disk.value = data as Disk;
    });

</script>
<template>
    <ul class="w-full h-full px-2 pt-2 text-neutral-50 grid grid-cols-2 custom-rows gap-1 select-none">
        <li @dblclick="openDisk" class="row-span-1 w-full bg-zinc-800 cursor-pointer p-4 h-24 text-xs rounded flex gap-5 hover:bg-stone-800 col-span-2">
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

        <FavoriteDirectories @favorite-open="emitFavorite"></FavoriteDirectories>

        <li class="col-fix col-span-2 bg-zinc-800 rounded w-full row-span-3 h-max">
            <TaskManager></TaskManager>
        </li>
    </ul>
</template>

<style>
    .col-fix {
        grid-row: 4;
        display: block;
    }
    .custom-rows {
        grid-template-rows: repeat(3, max-content);
    }
</style>