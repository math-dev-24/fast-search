<script setup lang="ts">
import { NCollapse, NCollapseItem, NIcon, type SelectOption } from 'naive-ui';
import { OptionsOutline } from '@vicons/ionicons5';

const emit = defineEmits<{
  (e: 'handleSearch'): void;
  (e: 'reset'): void;
}>();

const props = defineProps<{
  typeFiles: SelectOption[];
  folders: SelectOption[];
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

      <div class="flex flex-col">
        <!-- File Types & Folders Row -->
        <div class="filter-row grid grid-cols-2 gap-1 my-0.5">
          <FileTypesFilter :type-files="props.typeFiles" @handle-search="() => emit('handleSearch')" />
          <FoldersFilter :folders="props.folders" @handle-search="() => emit('handleSearch')" />
        </div>

        <!-- Date & Size Row -->
        <div class="filter-row">
          <DateFilter @handle-search="() => emit('handleSearch')" />
          <SizeLimitFilter @handle-search="() => emit('handleSearch')" />
        </div>
      </div>

      <!-- Search Options Switches -->
      <SearchSwitches @handle-search="() => emit('handleSearch')" />

      <!-- Action Buttons -->
      <ActionsBar @reset="() => emit('reset')" />
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