<script setup lang="ts">
import type {File} from '../../types';
import {FolderOpen} from '@vicons/ionicons5';
import {NIcon, NText} from 'naive-ui';
import {formatDate, formatPath} from '../../shared';

defineProps<{
  folder: File;
}>();

const emit = defineEmits<{
  (e: 'openFile', path: string): void;
}>();

</script>

<template>
  <tr class="hover-slate-100 transition-all duration-200 cursor-pointer">
    <td>
      <NText class="block mb-1 overflow-hidden text-ellipsis whitespace-nowrap font-semibold">
        {{ folder.name }}
      </NText>
    </td>
    <td>
      <NText class="block overflow-hidden text-ellipsis whitespace-nowrap leading-tight text-xs text-gray-500"
             depth="3">
        {{ formatPath(folder.path) }}
      </NText>
    </td>
    <td>
      {{ formatDate(folder.created_at) }}
    </td>
    <td>
      <div @click="emit('openFile', folder.path)" class="flex items-center gap-2 cursor-pointer hover:text-blue-500">
        <NIcon size="18" color="#3b82f6">
          <FolderOpen/>
        </NIcon>
        <NText>Ouvrir</NText>
      </div>
    </td>
  </tr>
</template>

<style scoped>

</style>