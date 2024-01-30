<script setup lang="ts">
    import { invoke } from '@tauri-apps/api';
    import { onMounted, ref } from 'vue';

    type sysInfo = {
        cpuUsage: string
        ramUsage: string
        swapUsage: string
        ramNumbers: Array<number>
        swpNumbers: Array<number>
    };

    let taskManagerData = ref<sysInfo | null>(null);
    const TIME_FOR_TSK_UPDATE = 2_000; //ms

    function bytesToGiB(bytes: number): number {
        return bytes / Math.pow(1024, 3);
    }

    setInterval(() => {
        invoke("start_tsk_mgr").then((data) => {
            taskManagerData.value = data as sysInfo;
        })
    }, TIME_FOR_TSK_UPDATE);

    onMounted(() => {
        taskManagerData.value = {
            cpuUsage: "0",
            ramUsage: "0",
            swapUsage: "0",
            ramNumbers: [0, 0],
            swpNumbers: [0, 0]
        }
        invoke('start_tsk_mgr').then((data) => {
            taskManagerData.value = data as sysInfo;
        })
    })
</script>

<template>
    <ul>
        <li class="w-full bg-zinc-800 cursor-pointer p-4 h-24 text-xs rounded flex gap-5 ">
            <div class="w-16 flex items-center h-full ">
                <img src="@assets/cpu.png" alt="">
            </div>
            <section class="flex flex-col w-full justify-around">
                <div class="w-full flex justify-between">
                    <strong class="text-lg">CPU</strong>
                    <strong class="text-lg">{{ taskManagerData?.cpuUsage }}%</strong>
                </div>
                <div class="w-full bg-white h-5 border-2 border-white">
                    <section class="bg-blue-500 h-full"
                        :style="{width: `${taskManagerData?.cpuUsage}%`}">
                    </section>
                </div>
            </section>
        </li>
        <li class="w-full bg-zinc-800 cursor-pointer p-4 h-24 text-xs rounded flex gap-5" >
            <div class="w-16 flex items-center h-full ">
                <img src="@assets/ram.png" alt="">                
            </div>
            <section class="flex flex-col w-full justify-around">
                <div class="w-full flex flex-col justify-between">
                    <section class="flex justify-between">
                        <strong class="text-lg">RAM</strong>
                        <strong class="text-lg">{{ taskManagerData?.ramUsage}}%</strong>
                    </section>
                    <section class="flex justify-between">
                        <p class="text-sm">
                            {{ bytesToGiB(taskManagerData?.ramNumbers[0]!).toFixed(2) }}
                            GiB
                        </p>
                        <p class="text-sm">
                            {{ bytesToGiB(taskManagerData?.ramNumbers[1]!).toFixed(2) }}
                            GiB
                        </p>
                    </section>
                </div>
                <div class="w-full bg-white h-full border-2 border-white">
                    <section class="bg-purple-500 h-full"
                        :style="{width: `${taskManagerData?.ramUsage}%`}">
                    </section>
                </div>
            </section>
        </li>
        <li class="w-full bg-zinc-800 cursor-pointer p-4 h-24 text-xs rounded flex gap-5" >
            <div class="w-16 flex items-center h-full ">
                <img src="@assets/swp.png" alt="">                
            </div>
            <section class="flex flex-col w-full justify-around">
                <div class="w-full flex flex-col justify-between">
                    <section class="flex justify-between">
                        <strong class="text-lg">Swap</strong>
                        <strong class="text-lg">{{ taskManagerData?.swapUsage}}%</strong>
                    </section>
                    <section class="flex justify-between">
                        <p class="text-sm">
                            {{ bytesToGiB(taskManagerData?.swpNumbers[0]!).toFixed(2) }}
                            GiB
                        </p>
                        <p class="text-sm">
                            {{ bytesToGiB(taskManagerData?.swpNumbers[1]!).toFixed(2) }}
                            GiB
                        </p>
                    </section>
                </div>
                <div class="w-full bg-white h-full border-2 border-white">
                    <section class="bg-blue-500 h-full"
                        :style="{width: `${taskManagerData?.swapUsage}%`}">
                    </section>
                </div>
            </section>
        </li>
    </ul>
</template>