<script setup lang="ts">
import { NCollapse, NCollapseItem, NIcon, NText, type SelectOption } from 'naive-ui';
import { OptionsOutline } from '@vicons/ionicons5';

const emit = defineEmits<{
  (e: 'handleSearch'): void;
  (e: 'reset'): void;
}>();

const props = defineProps<{
  typeFiles: SelectOption[];
  folders: SelectOption[];
  loadingFilters?: boolean;
}>();
</script>

<template>
  <NCollapse :default-expanded-names="['filters']" class="search-collapse mt-2">
    <NCollapseItem name="filters" title="Options de recherche">
      <template #header-extra>
        <NIcon class="opacity-60" size="16">
          <OptionsOutline />
        </NIcon>
      </template>

      <div class="flex flex-col space-y-4">
        <!-- Section: Filtres par type et emplacement -->
        <div>
          <NText class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2 block">
            Filtres par type et emplacement
          </NText>
          <div class="filter-row grid grid-cols-2 gap-1 my-0.5">
            <FileTypesFilter 
              :type-files="props.typeFiles" 
              :loading-filters="props.loadingFilters"
              @handle-search="() => emit('handleSearch')" 
            />
            <FoldersFilter 
              :folders="props.folders" 
              :loading-filters="props.loadingFilters"
              @handle-search="() => emit('handleSearch')" 
            />
          </div>
        </div>

        <!-- Section: Filtres par date et taille -->
        <div>
          <NText class="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-2 block">
            Filtres par date et taille
          </NText>
          <div class="filter-row grid grid-cols-2 gap-1">
            <DateFilter @handle-search="() => emit('handleSearch')" />
            <SizeLimitFilter @handle-search="() => emit('handleSearch')" />
          </div>
        </div>
      </div>

      <!-- Search Options Switches -->
      <SearchSwitches @handle-search="() => emit('handleSearch')" />

      <!-- Action Buttons -->
      <ActionsBar 
        @reset="() => emit('reset')" 
        @search="() => emit('handleSearch')"
      />
    </NCollapseItem>
  </NCollapse>
</template>

<script lang="ts">
import FileTypesFilter from './FileTypesFilter.vue';
import FoldersFilter from './FoldersFilter.vue';
import DateFilter from './DateFilter.vue';
import SizeLimitFilter from './SizeLimitFilter.vue';
import SearchSwitches from './SearchSwitches.vue';
import ActionsBar from './ActionsBar.vue';

export default {
  components: {
    FileTypesFilter,
    FoldersFilter,
    DateFilter,
    SizeLimitFilter,
    SearchSwitches,
    ActionsBar
  }
};
</script>