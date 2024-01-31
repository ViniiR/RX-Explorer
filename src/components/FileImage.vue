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
            return 'src/assets/directory.png'
        }
        if (file[file.lastIndexOf('\\') + 1] === ".") {
            return 'src/assets/config.png'
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
            return 'src/assets/imageFile.png'
        }
        else if (COMMON_ZIP_EXT.includes(ext!.toUpperCase())) {
            return 'src/assets/zip.png'
        }
        else if (COMMON_CONFIG_EXT.includes(ext!.toUpperCase())) {
            return 'src/assets/config.png'
        }
        else if (COMMON_VIDEO_EXT.includes(ext!.toUpperCase())) {
            return 'src/assets/video.png'
        }
        else if (COMMON_MUSIC_EXT.includes(ext!.toUpperCase())) {
            return 'src/assets/music.png'
        }
        else if (COMMON_BINARY_EXT.includes(ext!.toUpperCase())) {
            return ('src/assets/exe.png')
        } 
        else if (ext === "pdf") {
            return 'src/assets/pdf.png'
        }
        else if (ext === "ceres") {
            return 'src/assets/ceresIcon.svg'
        }
        else if (ext === "c") {
            return 'src/assets/cLogo.png'
        }
        else if (ext === "rs") {
            return 'src/assets/rust-logo-blk.svg'
        }
        else if (ext === "lnk") {
            return "src/assets/linkFile.png"
        }
        else {
            return ('src/assets/txt_10260761.png')
        }
    }

    onMounted(async () => {
        imageSrc.value = await checkFileExtension(file);
    })

</script>

<template>
    <img class="h-5 w-5" :src="imageSrc!" alt="">
</template>