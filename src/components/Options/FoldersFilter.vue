<script setup lang="ts">
import { NCard, NIcon, NSelect, type SelectOption } from 'naive-ui';
import { FolderOutline } from '@vicons/ionicons5';
import { useSearchStore } from '../../shared';

const props = defineProps<{ 
  folders: SelectOption[];
  loadingFilters?: boolean;
}>();
const emit = defineEmits<{ (e: 'handleSearch'): void }>();

const searchStore = useSearchStore();
</script>

<template>
  <NCard class="filter-card" size="small">
    <template #header>
      <div class="filter-header flex items-center gap-2">
        <NIcon class="text-orange-500" size="16">
          <FolderOutline />
        </NIcon>
        <span>Dossiers</span>
      </div>
    </template>
    <NSelect
      v-model:value="searchStore.query.filters.folders"
      :disabled="searchStore.in_loading || props.loadingFilters"
      :loading="props.loadingFilters"
      :options="props.folders"
      filterable
      max-tag-count="responsive"
      multiple
      placeholder="Tous les dossiers"
      @update:value="emit('handleSearch')"
    />
  </NCard>
</template>
