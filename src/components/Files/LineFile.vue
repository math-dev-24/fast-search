<script lang="ts" setup>
import type {File} from '../../types';
import {FolderOpen} from '@vicons/ionicons5';
import {NButton, NIcon, NText} from 'naive-ui';
import {formatDate, formatPath} from '../../shared';

defineProps<{
  file: File;
}>();

const emit = defineEmits<{
  (e: 'openFile', path: string): void;
  (e: 'copyPath', path: string): void;
  (e: 'previewFile', file: File): void;
}>();

</script>

<template>
  <tr class="hover-slate-100 transition-all duration-200 cursor-pointer">
    <td>
      <NText class="block mb-1 overflow-hidden text-ellipsis whitespace-nowrap font-semibold">
        {{ file.name }}
      </NText>
    </td>
    <td>
      <NText class="block overflow-hidden text-ellipsis whitespace-nowrap leading-tight text-xs text-gray-500"
             depth="3">
        {{ formatPath(file.path) }}
      </NText>
    </td>
    <td>
      {{ formatDate(file.created_at) }}
    </td>
    <td>
      {{ formatDate(file.accessed_at) }}
    </td>
    <td class="flex gap-1 items-center">
      <NButton
          class="flex gap-2 items-center"
          size="small"
          @click="emit('openFile', file.path)"
      >
        <NIcon color="#3b82f6" size="18">
          <FolderOpen/>
        </NIcon>
        <NText>Ouvrir</NText>
      </NButton>
      <NButton
          class="flex gap-2 items-center"
          size="small"
          @click="() => emit('previewFile', file)"
      >
        <NIcon color="#3b82f6" size="18">
          <FolderOpen/>
        </NIcon>
        <NText>Détails</NText>
      </NButton>
      <NButton
          class="flex gap-2 items-center"
          size="small"
          @click="() => emit('copyPath', file.path)"
      >
        <NIcon color="#3b82f6" size="18">
          <FolderOpen/>
        </NIcon>
        <NText>Copy</NText>
      </NButton>
    </td>
  </tr>
</template>
