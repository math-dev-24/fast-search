<script setup lang="ts">
import type { File } from '../types';
import { FolderOpen } from '@vicons/ionicons5';
import { NIcon, NCard, NText } from 'naive-ui';
import { formatPath } from '../shared/pathFormat';

defineProps<{
    file: File;
}>();

const emit = defineEmits<{
    (e: 'openFile', path: string): void;
}>();

</script>

<template>
    <NCard 
        hoverable 
        @click="emit('openFile', file.path)" 
        class="cursor-pointer transition-all duration-200 hover:shadow-lg hover:-translate-y-0.5 rounded-xl border border-gray-200 hover:border-blue-500"
        size="small"
    >
        <div class="flex items-center gap-3 py-2">
            <div class="flex-shrink-0 flex items-center justify-center w-12 h-12 bg-gradient-to-br from-blue-100 to-blue-200 rounded-xl">
                <NIcon size="32" color="#3b82f6">
                    <FolderOpen />
                </NIcon>
            </div>
            
            <div class="flex-1 min-w-0">
                <NText class="block mb-1 overflow-hidden text-ellipsis whitespace-nowrap font-semibold">
                    {{ file.name }}
                </NText>
                
                <NText class="block overflow-hidden text-ellipsis whitespace-nowrap leading-tight text-xs text-gray-500" depth="3">
                    {{ formatPath(file.path) }}
                </NText>
            </div>
        </div>
    </NCard>
</template>