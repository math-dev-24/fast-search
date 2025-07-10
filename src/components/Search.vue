<template>
    <div>
        <div class="space-y-6">
            <div class="relative">
                <NInput v-model:value="modelValue.search" placeholder="Rechercher..." :loading="modelValue.inLoading"
                    :disabled="modelValue.inLoading" @keyup.enter="emit('search')" tabindex="1" size="large"
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
                    <NSelect v-model:value="modelValue.types" :options="typeFiles" multiple @update:value="handleSearch"
                        :loading="modelValue.inLoading" :disabled="modelValue.inLoading" filterable
                        placeholder="Sélectionner les types..." class="w-full" />
                </div>

                <div class="space-y-2">
                    <label class="text-sm font-medium">Dossiers</label>
                    <NSelect v-model:value="modelValue.folders" :options="folders" multiple @update:value="handleSearch"
                        :loading="modelValue.inLoading" :disabled="modelValue.inLoading" filterable
                        placeholder="Sélectionner les dossiers..." class="w-full" />
                </div>

                <div class="space-y-2">
                    <label class="text-sm font-medium">Filtrer par date de <span class="underline" @click="updateDateMode">{{ modelValue.dateMode === 'create' ? 'création' : 'modification' }}</span> :</label>
                    <NDatePicker v-model:value="modelValue.dateRange" type="daterange"  clearable @update:value="handleDateChange" />
                    <div class="flex items-center gap-2">
                        <NButton 
                            v-for="shortcut in listShortcuts" 
                            :key="shortcut" 
                            @click="() => { updateDateRange(shortcut) }" 
                            :disabled="isSelectedDate(modelValue.dateRange, getDateShortcuts(shortcut))"
                            tertiary
                            :type="isSelectedDate(modelValue.dateRange, getDateShortcuts(shortcut)) ? 'info' : 'default'"
                            class="flex-1"
                        >
                            {{ shortcut }}
                        </NButton>
                    </div>
                </div>

                <div class="space-y-2">
                    <label class="text-sm font-medium">Taille (en Mo)</label>
                    <div class="flex items-center space-x-2">
                        <NInputNumber v-model:value="modelValue.sizeLimit[0]" placeholder="Min" :min="0"
                            :max="modelValue.sizeLimit[1]" @update:value="() => handleSizeChange()" />
                        <span class="text-gray-500">-</span>
                        <NInputNumber v-model:value="modelValue.sizeLimit[1]" placeholder="Max"
                            :min="modelValue.sizeLimit[0] || 0" @update:value="() => handleSizeChange()" />
                    </div>
                </div>


                <NSpace vertical class="col-span-2 mt-2">
                    <NSpace>
                        <NSwitch v-model:value="modelValue.isDir" @update:value="handleSearch">
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
                        <NSwitch v-model:value="modelValue.autoSubmit">
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
                    </NSpace>

                    <div class="w-full grid grid-cols-3">
                        <NButton @click="() => { syncTypeFiles(); syncFolders() }" :disabled="modelValue.inLoading"
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
                        <NButton @click="emit('search')" type="primary" :disabled="modelValue.inLoading">
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
import { Search, SyncCircleOutline, Refresh, Folder, Document, RefreshCircle } from '@vicons/ionicons5';
import { useDebounceFn } from '@vueuse/core';
import { useDate } from '../composables/useDate';

const modelValue = defineModel<{
    search: string;
    types: string[];
    folders: string[];
    isDir: boolean;
    inLoading: boolean;
    showPath: boolean;
    autoSubmit: boolean;
    dateMode: 'create' | 'modify';
    sizeLimit: [number, number];
    dateRange: [number, number];
}>({
    required: true
});

const typeFiles = ref<SelectOption[]>([]);
const folders = ref<SelectOption[]>([]);
const { dateShortcuts, listShortcuts, isSelectedDate } = useDate();

const emit = defineEmits<{
    (e: 'search'): void;
    (e: 'reset'): void;
}>();

onMounted(async () => {
    await syncTypeFiles();
    await syncFolders();
})

const syncTypeFiles = async () => {
    modelValue.value.inLoading = true;
    try {
        const result = await invoke<string[]>('get_type_files');
        typeFiles.value = result.map(type => ({
            label: type,
            value: type
        }));
    } catch (error) {
        console.error(error);
    } finally {
        modelValue.value.inLoading = false;
    }
}

const syncFolders = async () => {
    modelValue.value.inLoading = true;
    try {
        const result = await invoke<string[]>('get_all_folders');
        folders.value = result.map(folder => ({
            label: folder,
            value: folder
        }));
    } catch (error) {
        console.error(error);
    } finally {
        modelValue.value.inLoading = false;
    }
}

const emitSearchDebounced = useDebounceFn(() => emit('search'), 500);

const handleSearch = () => {
    if (modelValue.value.autoSubmit && (modelValue.value.search.length > 0 || modelValue.value.types.length > 0 || modelValue.value.folders.length > 0) && !modelValue.value.inLoading) {
        emitSearchDebounced();
    }
}

const handleDateChange = () => {
    handleSearch();
}

const handleSizeChange = () => {
    handleSearch();
}

watch(() => modelValue.value.search, () => {
    handleSearch();
});

const getDateShortcuts = (shortcut: string) => {
    return dateShortcuts[shortcut as keyof typeof dateShortcuts]();
}

const updateDateRange = (shortcut: string) => {
    modelValue.value.dateRange = getDateShortcuts(shortcut);
    handleDateChange();
}

const updateDateMode = () => {
    modelValue.value.dateMode = modelValue.value.dateMode === 'create' ? 'modify' : 'create';
    handleDateChange();
}

</script>