<script setup lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMounted, ref } from 'vue';

    type ImgProps = {
        file: string;
    }

    let imageSrc = ref<string | null>(null)
    const props = defineProps<ImgProps>();
    const file = props.file
    
    async function checkFileExtension(file: string): Promise<string> {
        const isDir = await invoke("is_dir", {file});
        if (isDir) {
            return (await import('../assets/directory.png')).default
        }
        if (file[file.lastIndexOf('\\') + 1] === ".") {
            return (await import('../assets/config.png')).default
        }
        const ext = file.split(".").pop()
        const COMMON_IMG_EXT = 
            ["JPEG", "JPG", "PNG", "GIF", "SVG", "TIFF", "BMP", "WEBP", "ICO"];
        const COMMON_ZIP_EXT = 
            ["ZIP", "RAR", "7Z", "TAR", "GZ", "BZ2"];
        const COMMON_MUSIC_EXT = 
            ["MP3", "WAV", "FLAC", "AAC", "OGG", "WMA", "M4A"];
        const COMMON_VIDEO_EXT = 
            ["MP4", "AVI", "MKV", "MOV", "WMV", "FLV", "WEBM", "MPEG"];
        const COMMON_CONFIG_EXT = 
            ["JSON", "YAML", "XML", "INI", "CFG", "TOML", "SYS"];
        const COMMON_BINARY_EXT = 
            ["EXE", "MSI", "DLL", "BIN"];
        
        if (COMMON_IMG_EXT.includes(ext!.toUpperCase())) {
            return (await import('../assets/imageFile.png')).default
        }
        else if (COMMON_ZIP_EXT.includes(ext!.toUpperCase())) {
            return (await import('../assets/zip.png')).default
        }
        else if (COMMON_CONFIG_EXT.includes(ext!.toUpperCase())) {
            return (await import('../assets/config.png')).default
        }
        else if (COMMON_VIDEO_EXT.includes(ext!.toUpperCase())) {
            return (await import('../assets/video.png')).default
        }
        else if (COMMON_MUSIC_EXT.includes(ext!.toUpperCase())) {
            return (await import('../assets/music.png')).default
        }
        else if (COMMON_BINARY_EXT.includes(ext!.toUpperCase())) {
            return (await import('../assets/exe.png')).default
        } 
        else if (ext === "pdf") {
            return (await import('../assets/pdf.png')).default
        }
        else if (ext === "ceres") {
            return (await import('../assets/ceresIcon.svg')).default
        }
        else if (ext === "c") {
            return (await import('../assets/cLogo.png')).default
        }
        else if (ext === "rs") {
            return (await import('../assets/rust-logo-blk.svg')).default
        }
        else if (ext === "lnk") {
            return (await import("../assets/linkFile.png")).default
        }
        else {
            return (await import('../assets/txt_10260761.png')).default
        }
    }

    async function setImageIcons(file: string) {
        imageSrc.value = await checkFileExtension(file);
    }

    onMounted(() => {
        setImageIcons(file);
    })

</script>

<template>
    <img class="h-5 w-5" :src="imageSrc!" alt="">
</template>