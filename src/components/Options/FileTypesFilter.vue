<script setup lang="ts">
import { NCard, NIcon, NSelect, type SelectOption } from 'naive-ui';
import { DocumentText } from '@vicons/ionicons5';
import { useSearchStore } from '../../shared';

const props = defineProps<{ typeFiles: SelectOption[] }>();
const emit = defineEmits<{ (e: 'handleSearch'): void }>();

const searchStore = useSearchStore();
</script>

<template>
  <NCard class="filter-card" size="small">
    <template #header>
      <div class="filter-header flex items-center gap-2">
        <NIcon class="text-blue-500" size="16">
          <DocumentText />
        </NIcon>
        <span>Types de fichiers</span>
      </div>
    </template>
    <NSelect
      v-model:value="searchStore.query.filters.file_types"
      :disabled="searchStore.in_loading"
      :loading="searchStore.in_loading"
      :options="props.typeFiles"
      filterable
      max-tag-count="responsive"
      multiple
      placeholder="Tous les types"
      @update:value="emit('handleSearch')"
    />
  </NCard>
</template>
