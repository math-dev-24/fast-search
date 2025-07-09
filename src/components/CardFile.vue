<script setup lang="ts">
import { computed } from 'vue';
import type { File } from '../types';
import { FileTrayFull, FolderOpen, Open, Eye, CopyOutline, EyeOutline } from '@vicons/ionicons5';
import { NIcon, NCard, NText, NButton, NFlex } from 'naive-ui';
import { formatPath } from '../shared/pathFormat';

const props = defineProps<{
    file: File;
    showPath: boolean;
}>();

const emit = defineEmits<{
    (e: 'openFile', path: string): void;
    (e: 'copyPath', path: string): void;
    (e: 'previewFile', file: File): void;
}>();

// Fonction pour déterminer l'icône et la couleur selon l'extension
const getFileIcon = (fileName: string) => {
    const extension = fileName.split('.').pop()?.toLowerCase();
    
    const iconMap: Record<string, { icon: any, color: string, bgColor: string }> = {
        'pdf': { icon: FileTrayFull, color: '#ef4444', bgColor: 'linear-gradient(135deg, #fee2e2 0%, #fecaca 100%)' },
        'doc': { icon: FileTrayFull, color: '#2563eb', bgColor: 'linear-gradient(135deg, #dbeafe 0%, #bfdbfe 100%)' },
        'docx': { icon: FileTrayFull, color: '#2563eb', bgColor: 'linear-gradient(135deg, #dbeafe 0%, #bfdbfe 100%)' },
        'txt': { icon: FileTrayFull, color: '#6b7280', bgColor: 'linear-gradient(135deg, #f3f4f6 0%, #e5e7eb 100%)' },
        'jpg': { icon: FileTrayFull, color: '#10b981', bgColor: 'linear-gradient(135deg, #d1fae5 0%, #a7f3d0 100%)' },
        'jpeg': { icon: FileTrayFull, color: '#10b981', bgColor: 'linear-gradient(135deg, #d1fae5 0%, #a7f3d0 100%)' },
        'png': { icon: FileTrayFull, color: '#10b981', bgColor: 'linear-gradient(135deg, #d1fae5 0%, #a7f3d0 100%)' },
        'mp4': { icon: FileTrayFull, color: '#8b5cf6', bgColor: 'linear-gradient(135deg, #ede9fe 0%, #ddd6fe 100%)' },
        'mp3': { icon: FileTrayFull, color: '#f59e0b', bgColor: 'linear-gradient(135deg, #fef3c7 0%, #fde68a 100%)' },
        'zip': { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        'rar': { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        '7z': { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        'tar': { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        'gz': { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        'bz2': { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        'xz': { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        'exe': { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        'msi': { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        'ts': { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        'tsx': { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        "js": { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        "css": { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        "html": { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        "json": { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        "xml": { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        "csv": { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
        "xlsx": { icon: FileTrayFull, color: '#f97316', bgColor: 'linear-gradient(135deg, #fed7aa 0%, #fdba74 100%)' },
    };
    
    return iconMap[extension || ''] || { icon: FileTrayFull, color: '#6b7280', bgColor: 'linear-gradient(135deg, #f3f4f6 0%, #e5e7eb 100%)' };
};

const fileIcon = computed(() => getFileIcon(props.file.name));

// Vérifier si le fichier peut être prévisualisé
const canPreview = computed(() => {
    const extension = props.file.name.split('.').pop()?.toLowerCase();
    const previewableExtensions = ['pdf', 'jpg', 'jpeg', 'png', 'gif', 'webp', 'svg', 'txt', 'md', 'json', 'xml', 'csv', 'log', 'ini', 'conf', 'cfg'];
    return previewableExtensions.includes(extension || '');
});

</script>

<template>
    <NCard 
        hoverable
        class="cursor-pointer transition-all duration-200 hover:shadow-lg hover:-translate-y-0.5 rounded-xl border border-gray-200 hover:border-primary mb-3"
        size="small"
    >
        <div class="flex items-center gap-3 py-2">
            <div class="flex-shrink-0 flex items-center justify-center w-10 h-10 rounded-lg" :style="{ background: fileIcon.bgColor }">
                <NIcon size="24" :color="fileIcon.color">
                    <component :is="fileIcon.icon" />
                </NIcon>
            </div>
            
            <div class="flex-1 min-w-0">
                <NText class="block mb-1 overflow-hidden text-ellipsis whitespace-nowrap font-semibold">
                    {{ file.name }}
                </NText>
                
                <NText v-if="showPath" class="flex items-center overflow-hidden text-ellipsis whitespace-nowrap leading-tight text-xs text-gray-500" depth="3">
                    <NIcon size="12" class="mr-1">
                        <FolderOpen />
                    </NIcon>
                    {{ formatPath(file.path) }}
                </NText>
            </div>
        </div>
        <template #footer>
            <NFlex x-gap="10">
                <NButton v-if="canPreview" type="info" size="small" @click="() => emit('previewFile', file)" tertiary>
                    <NIcon size="16">
                        <EyeOutline />
                    </NIcon>
                </NButton>
                <NButton type="primary" size="small" @click="emit('openFile', file.path)" tertiary>
                    <NIcon size="16">
                        <Open />
                    </NIcon>
                </NButton>
                <NButton type="warning" size="small" @click="emit('copyPath', file.path)" tertiary>
                    <NIcon size="16">
                        <CopyOutline />
                    </NIcon>
                </NButton>
            </NFlex>
        </template>
    </NCard>
</template>