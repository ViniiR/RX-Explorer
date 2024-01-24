<script setup lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { ref } from 'vue';
    import { useRouter } from 'vue-router';

    let Disk = ref<Disk | null>(null);
    let CurrentDirectoryContents = ref<string | null>(null);
    const router = useRouter();
    const emit = defineEmits(['diskOpen'])

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
</script>
<template>
    <ul class="pt-14 w-full min-h-28 px-2 text-neutral-50 flex items-center select-none">
        <li @dblclick="openDisk" class="w-1/2 bg-zinc-800 cursor-pointer p-4 h-24 text-xs rounded flex gap-5 hover:bg-stone-800">
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
    </ul>
</template>