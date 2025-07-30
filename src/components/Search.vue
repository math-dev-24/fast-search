<template>
    <div>
        <div class="space-y-6">
            <div class="relative">
                <NInput v-model:value="searchStore.query.text" placeholder="Rechercher..." :loading="searchStore.inLoading"
                    :disabled="searchStore.inLoading" @keyup.enter="searchStore.searchFiles" tabindex="1" size="large"
                    class="text-lg">
                    <template #prefix>
                        <NIcon size="20">
                            <Search />
                        </NIcon>
                    </template>
                </NInput>
            </div>

            <div class="grid grid-cols-2 gap-4">
                <div class="space-y-2">
                    <label class="text-sm font-medium">Types de fichiers</label>
                    <NSelect v-model:value="searchStore.query.filters.file_types" :options="typeFiles" multiple @update:value="handleSearch"
                        :loading="searchStore.inLoading" :disabled="searchStore.inLoading" filterable
                        placeholder="Sélectionner les types..." class="w-full" />
                </div>

                <div class="space-y-2">
                    <label class="text-sm font-medium">Dossiers</label>
                    <NSelect v-model:value="searchStore.query.filters.folders" :options="folders" multiple @update:value="handleSearch"
                        :loading="searchStore.inLoading" :disabled="searchStore.inLoading" filterable
                        placeholder="Sélectionner les dossiers..." class="w-full" />
                </div>

                <div class="space-y-2">
                    <label class="text-sm font-medium">Filtrer par date de <span class="underline" @click="updateDateMode">{{ searchStore.query.filters.date_mode === DateMode.CREATE ? 'création' : 'modification' }}</span> :</label>
                    <NDatePicker v-model:value="searchStore.query.filters.date_range" type="daterange"  clearable @update:value="handleDateChange" />
                    <div class="flex items-center flex-wrap gap-2">
                        <NButton 
                            v-for="shortcut in listShortcuts" 
                            :key="shortcut" 
                            @click="() => { updateDateRange(shortcut) }" 
                            :disabled="isSelectedDate(searchStore.query.filters.date_range, getDateShortcuts(shortcut))"
                            tertiary
                            :type="isSelectedDate(searchStore.query.filters.date_range, getDateShortcuts(shortcut)) ? 'info' : 'default'"
                            class="flex-1"
                        >
                            {{ shortcut }}
                        </NButton>
                    </div>
                </div>

                <div class="space-y-2">
                    <label class="text-sm font-medium">Taille (en Mo)</label>
                    <div class="flex items-center space-x-2 w-full">
                        <NInputNumber v-model:value="searchStore.query.filters.size_limit[0]" placeholder="Min" :min="0"
                            :max="searchStore.query.filters.size_limit[1]" @update:value="() => handleSizeChange()" />
                        <span class="text-gray-500">-</span>
                        <NInputNumber v-model:value="searchStore.query.filters.size_limit[1]" placeholder="Max"
                            :min="searchStore.query.filters.size_limit[0] || 0" @update:value="() => handleSizeChange()" />
                    </div>
                    <div>
                        <label class="text-sm font-medium">Limite de résultats</label>
                        <NInputNumber v-model:value="searchStore.query.limit" placeholder="Limite" :min="1" :max="10000" />
                    </div>
                </div>






                <NSpace vertical class="col-span-2 mt-2">
                    <NSpace>
                        <NSwitch v-model:value="searchStore.query.filters.is_dir" @update:value="handleSearch">
                            <template #checked>
                                <div class="flex items-center space-x-2">
                                    <NIcon size="16" class="text-blue-600">
                                        <Folder />
                                    </NIcon>
                                    <span class="text-sm font-medium">Dossiers uniquement</span>
                                </div>
                            </template>
                            <template #unchecked>
                                <div class="flex items-center space-x-2">
                                    <NIcon size="16" class="text-gray-600">
                                        <Document />
                                    </NIcon>
                                    <span class="text-sm font-medium">Tous les fichiers</span>
                                </div>
                            </template>
                        </NSwitch>
                        <NSwitch v-model:value="searchStore.auto_submit">
                            <template #checked>
                                <div class="flex items-center space-x-2">
                                    <NIcon size="16">
                                        <Refresh />
                                    </NIcon>
                                    <span class="text-sm font-medium">Recherche automatique</span>
                                </div>
                            </template>
                            <template #unchecked>
                                <div class="flex items-center space-x-2">
                                    <NIcon size="16">
                                        <RefreshCircle />
                                    </NIcon>
                                    <span class="text-sm font-medium">Recherche manuelle</span>
                                </div>
                            </template>
                        </NSwitch>
                        <NSwitch v-model:value="searchStore.query.filters.search_in_content">
                            <template #checked>
                                <div class="flex items-center space-x-2">
                                    <NIcon size="16" class="text-green-600">
                                        <DocumentText />
                                    </NIcon>
                                    <span class="text-sm font-medium">Recherche dans le contenu</span>
                                </div>
                            </template>
                            <template #unchecked>
                                <div class="flex items-center space-x-2">
                                    <NIcon size="16" class="text-gray-600">
                                        <Document />
                                    </NIcon>
                                    <span class="text-sm font-medium">Recherche dans les métadonnées</span>
                                </div>
                            </template>
                        </NSwitch>
                    </NSpace>

                    <div class="w-full grid grid-cols-3">
                        <NButton @click="() => { syncTypeFiles(); syncFolders() }" :disabled="searchStore.inLoading"
                            tertiary type="info" class="flex-1">
                            <NIcon size="16" class="mr-2">
                                <SyncCircleOutline />
                            </NIcon>
                            Sync
                        </NButton>
                        <NButton @click="emit('reset')" tertiary type="warning" class="flex-1">
                            <NIcon size="16" class="mr-2">
                                <Refresh />
                            </NIcon>
                            Reset
                        </NButton>
                        <NButton @click="emit('search')" type="primary" :disabled="searchStore.inLoading">
                            <NIcon size="16" class="mr-2">
                                <Search />
                            </NIcon>
                            Rechercher
                        </NButton>
                    </div>
                </NSpace>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { NInput, NButton, NIcon, NSelect, type SelectOption, NSwitch, NSpace, NInputNumber, NDatePicker } from 'naive-ui';
import { Search, SyncCircleOutline, Refresh, Folder, Document, RefreshCircle, DocumentText } from '@vicons/ionicons5';
import { useDebounceFn } from '@vueuse/core';
import { useDate } from '../composables/useDate';
import { useSearchStore } from '../shared/store/search';
import { DateMode } from '../types/search';

const typeFiles = ref<SelectOption[]>([]);
const folders = ref<SelectOption[]>([]);

const { dateShortcuts, listShortcuts, isSelectedDate } = useDate();

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
    searchStore.inLoading = true;
    try {
        const result = await invoke<string[]>('get_all_types');
        typeFiles.value = result.map(type => ({
            label: type,
            value: type
        }));
    } catch (error) {
        console.error(error);
    } finally {
        searchStore.inLoading = false;
    }
}

const syncFolders = async () => {
    searchStore.inLoading = true;
    try {
        const result = await invoke<string[]>('get_all_folders');
        folders.value = result.map(folder => ({
            label: folder,
            value: folder
        }));
    } catch (error) {
        console.error(error);
    } finally {
        searchStore.inLoading = false;
    }
}

const emitSearchDebounced = useDebounceFn(() => emit('search'), 500);

const handleSearch = () => {
    if (searchStore.auto_submit && (searchStore.query.text.length > 0 || searchStore.query.filters.file_types.length > 0 || searchStore.query.filters.folders.length > 0) && !searchStore.inLoading) {
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