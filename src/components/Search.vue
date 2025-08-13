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

    <!-- Search Options -->
    <NCollapse :default-expanded-names="['filters']" class="search-collapse mt-2">
      <NCollapseItem name="filters" title="Options de recherche">
        <template #header-extra>
          <NIcon class="opacity-60" size="16">
            <OptionsOutline/>
          </NIcon>
        </template>

        <div class="flex flex-col">
          <!-- File Types & Folders Row -->
          <div class="filter-row grid grid-cols-2 gap-1 my-0.5">
            <NCard class="filter-card" size="small">
              <template #header>
                <div class="filter-header flex items-center gap-2">
                  <NIcon class="text-blue-500" size="16">
                    <DocumentText/>
                  </NIcon>
                  <span>Types de fichiers</span>
                </div>
              </template>
              <NSelect
                  v-model:value="searchStore.query.filters.file_types"
                  :disabled="searchStore.in_loading"
                  :loading="searchStore.in_loading"
                  :options="typeFiles"
                  filterable
                  max-tag-count="responsive"
                  multiple
                  placeholder="Tous les types"
                  @update:value="handleSearch"
              />
            </NCard>

            <NCard class="filter-card" size="small">
              <template #header>
                <div class="filter-header flex items-center gap-2">
                  <NIcon class="text-orange-500" size="16">
                    <FolderOutline/>
                  </NIcon>
                  <span>Dossiers</span>
                </div>
              </template>
              <NSelect
                  v-model:value="searchStore.query.filters.folders"
                  :disabled="searchStore.in_loading"
                  :loading="searchStore.in_loading"
                  :options="folders"
                  filterable
                  max-tag-count="responsive"
                  multiple
                  placeholder="Tous les dossiers"
                  @update:value="handleSearch"
              />
            </NCard>
          </div>

          <!-- Date & Size Row -->
          <div class="filter-row">
            <NCard class="filter-card date-card" size="small">
              <template #header>
                <div class="filter-header">
                  <NIcon class="text-green-500" size="16">
                    <CalendarOutline/>
                  </NIcon>
                  <span>
                                        Date de 
                                        <NButton
                                            class="date-mode-btn"
                                            text
                                            type="primary"
                                            @click="updateDateMode"
                                        >
                                            {{
                                            searchStore.query.filters.date_mode === DateMode.CREATE ? 'création' : 'modification'
                                          }}
                                        </NButton>
                                    </span>
                </div>
              </template>
              <NSpace size="small" vertical>
                <NDatePicker
                    v-model:value="searchStore.query.filters.date_range"
                    clearable
                    placeholder="Sélectionner une période"
                    type="daterange"
                    @update:value="handleDateChange"
                />
                <NSpace class="date-shortcuts" size="small">
                  <NButton
                      v-for="shortcut in listShortcuts"
                      :key="shortcut"
                      :disabled="isSelectedDate(searchStore.query.filters.date_range, getDateShortcuts(shortcut))"
                      :type="isSelectedDate(searchStore.query.filters.date_range, getDateShortcuts(shortcut)) ? 'primary' : 'default'"
                      secondary
                      size="tiny"
                      @click="() => { updateDateRange(shortcut) }"
                  >
                    {{ shortcut }}
                  </NButton>
                </NSpace>
              </NSpace>
            </NCard>

            <NCard class="filter-card" size="small">
              <template #header>
                <div class="filter-header">
                  <NIcon class="text-purple-500" size="16">
                    <Resize/>
                  </NIcon>
                  <span>Taille & Limites</span>
                </div>
              </template>
              <NSpace size="small" vertical>
                <div class="size-inputs">
                  <span class="size-label">Taille (Mo)</span>
                  <div class="size-range grid grid-cols-2 gap-1">
                    <NInputNumber
                        v-model:value="searchStore.query.filters.size_limit[0]"
                        :max="searchStore.query.filters.size_limit[1]"
                        :min="0"
                        aria-label="Min"
                        placeholder="Min"
                        size="small"
                        @update:value="handleSizeChange"
                    />
                    <NInputNumber
                        v-model:value="searchStore.query.filters.size_limit[1]"
                        :min="searchStore.query.filters.size_limit[0] || 0"
                        aria-label="Max"
                        placeholder="Max"
                        size="small"
                        @update:value="handleSizeChange"
                    />
                  </div>
                </div>
                <div class="limit-input">
                  <span class="size-label">Limite de résultats</span>
                  <NInputNumber
                      v-model:value="searchStore.query.limit"
                      :max="10000"
                      :min="1"
                      placeholder="1000"
                      size="small"
                  />
                </div>
              </NSpace>
            </NCard>
          </div>
        </div>

        <!-- Search Options Switches -->
        <NDivider class="options-divider">Options de recherche</NDivider>
        <div class="search-options grid grid-cols-3">
          <div class="option-group">
            <NSwitch v-model:value="searchStore.query.filters.is_dir" size="medium" @update:value="handleSearch">
              <template #checked>
                <NIcon class="text-blue-500" size="16">
                  <Folder/>
                </NIcon>
              </template>
              <template #unchecked>
                <NIcon class="text-gray-400" size="16">
                  <Document/>
                </NIcon>
              </template>
            </NSwitch>
            <div class="option-label">
              <span class="option-title">{{
                  searchStore.query.filters.is_dir ? 'Dossiers uniquement' : 'Tous les fichiers'
                }}</span>
              <span class="option-desc">{{
                  searchStore.query.filters.is_dir ? 'Rechercher dans les dossiers seulement' : 'Inclure fichiers et dossiers'
                }}</span>
            </div>
          </div>

          <div class="option-group">
            <NSwitch v-model:value="searchStore.auto_submit" size="medium">
              <template #checked>
                <NIcon class="text-green-500" size="16">
                  <Refresh/>
                </NIcon>
              </template>
              <template #unchecked>
                <NIcon class="text-gray-400" size="16">
                  <RefreshCircle/>
                </NIcon>
              </template>
            </NSwitch>
            <div class="option-label">
              <span class="option-title">{{
                  searchStore.auto_submit ? 'Recherche automatique' : 'Recherche manuelle'
                }}</span>
              <span class="option-desc">{{
                  searchStore.auto_submit ? 'Résultats en temps réel' : 'Recherche sur validation'
                }}</span>
            </div>
          </div>

          <div class="option-group">
            <NSwitch v-model:value="searchStore.query.filters.search_in_content" size="medium">
              <template #checked>
                <NIcon class="text-purple-500" size="16">
                  <DocumentText/>
                </NIcon>
              </template>
              <template #unchecked>
                <NIcon class="text-gray-400" size="16">
                  <Document/>
                </NIcon>
              </template>
            </NSwitch>
            <div class="option-label">
              <span class="option-title">{{
                  searchStore.query.filters.search_in_content ? 'Recherche dans le contenu' : 'Recherche dans les métadonnées'
                }}</span>
              <span class="option-desc">{{
                  searchStore.query.filters.search_in_content ? 'Analyser le contenu des fichiers' : 'Nom et propriétés seulement'
                }}</span>
            </div>
          </div>
        </div>

        <!-- Action Buttons -->
        <NDivider class="actions-divider"/>
        <div class="search-actions">
          <NButtonGroup>
            <NButton
                type="default"
                @click="emit('reset')"
            >
              <template #icon>
                <NIcon>
                  <Refresh/>
                </NIcon>
              </template>
              Réinitialiser
            </NButton>
          </NButtonGroup>
        </div>
      </NCollapseItem>
    </NCollapse>
  </div>
</template>

<script lang="ts" setup>
import {onMounted, ref, watch} from 'vue';
import {invoke} from '@tauri-apps/api/core';
import {
  NButton,
  NButtonGroup,
  NCard,
  NCollapse,
  NCollapseItem,
  NDatePicker,
  NDivider,
  NIcon,
  NInput,
  NInputNumber,
  NSelect,
  NSpace,
  NSwitch,
  type SelectOption
} from 'naive-ui';
import {
  CalendarOutline,
  Document,
  DocumentText,
  Folder,
  FolderOutline,
  OptionsOutline,
  Refresh,
  RefreshCircle,
  Resize,
  Search
} from '@vicons/ionicons5';
import {useDebounceFn} from '@vueuse/core';
import {useDate} from '../composables/useDate';
import {useSearchStore} from '../shared';
import {DateMode} from '../types';

const typeFiles = ref<SelectOption[]>([]);
const folders = ref<SelectOption[]>([]);

const {dateShortcuts, listShortcuts, isSelectedDate} = useDate();

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

const handleDateChange = () => {
  handleSearch();
}

const handleSizeChange = () => {
  handleSearch();
}

watch(() => searchStore.query.text, () => {
  handleSearch();
});

const getDateShortcuts = (shortcut: string) => {
  return dateShortcuts[shortcut as keyof typeof dateShortcuts]();
}

const updateDateRange = (shortcut: string) => {
  searchStore.query.filters.date_range = getDateShortcuts(shortcut);
  handleDateChange();
}

const updateDateMode = () => {
  searchStore.query.filters.date_mode = searchStore.query.filters.date_mode === DateMode.CREATE ? DateMode.MODIFY : DateMode.CREATE;
  handleDateChange();
}
</script>