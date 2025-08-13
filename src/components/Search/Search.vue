<template>
  <div class="search-container">
    <!-- Search Header -->
    <div class="search-header">
      <div class="search-input-wrapper">
        <NInput
            v-model:value="searchStore.query.text"
            :disabled="searchStore.in_loading"
            :loading="searchStore.in_loading"
            clearable
            placeholder="Rechercher des fichiers et dossiers..."
            size="large"
            tabindex="1"
            @keyup.enter="searchStore.searchFiles"
        >
          <template #prefix>
            <NIcon class="text-primary" size="20">
              <Search/>
            </NIcon>
          </template>
          <template #suffix>
            <NButton
                :disabled="searchStore.in_loading"
                class="search-btn"
                size="small"
                type="primary"
                @click="emit('search')"
            >
              <template #icon>
                <NIcon>
                  <Search/>
                </NIcon>
              </template>
            </NButton>
          </template>
        </NInput>
      </div>
    </div>

    <!-- Options (filters) -->
    <Options
      :type-files="typeFiles"
      :folders="folders"
      @handle-search="handleSearch"
      @reset="emit('reset')"
    />

  </div>
</template>

<script lang="ts" setup>
import {onMounted, ref, watch} from 'vue';
import {invoke} from '@tauri-apps/api/core';
import {
  NButton,
  NIcon,
  NInput,
  type SelectOption
} from 'naive-ui';
import {
  Search
} from '@vicons/ionicons5';
import {useDebounceFn} from '@vueuse/core';
import {useSearchStore} from '../../shared';
import Options from '../Options';

const typeFiles = ref<SelectOption[]>([]);
const folders = ref<SelectOption[]>([]);


const searchStore = useSearchStore();

const emit = defineEmits<{
  (e: 'search'): void;
  (e: 'reset'): void;
}>();

onMounted(async () => {
  await syncTypeFiles();
  await syncFolders();
})

const syncTypeFiles = async () => {
  searchStore.in_loading = true;
  try {
    const result = await invoke<string[]>('get_all_types');
    typeFiles.value = result.map(type => ({
      label: type,
      value: type
    }));
  } catch (error) {
    console.error(error);
  } finally {
    searchStore.in_loading = false;
  }
}

const syncFolders = async () => {
  searchStore.in_loading = true;
  try {
    const result = await invoke<string[]>('get_all_folders');
    folders.value = result.map(folder => ({
      label: folder,
      value: folder
    }));
  } catch (error) {
    console.error(error);
  } finally {
    searchStore.in_loading = false;
  }
}

const emitSearchDebounced = useDebounceFn(() => emit('search'), 500);

const handleSearch = () => {
  if (searchStore.auto_submit && (searchStore.query.text.length > 0 || searchStore.query.filters.file_types.length > 0 || searchStore.query.filters.folders.length > 0) && !searchStore.in_loading) {
    emitSearchDebounced();
  }
}

watch(() => searchStore.query.text, () => {
  handleSearch();
});

</script>