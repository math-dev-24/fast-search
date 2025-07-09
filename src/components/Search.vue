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

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                <div class="space-y-2">
                    <label class="text-sm font-medium">Types de fichiers</label>
                    <NSelect v-model:value="modelValue.types" :options="typeFiles" multiple
                        :loading="modelValue.inLoading" :disabled="modelValue.inLoading" filterable
                        placeholder="Sélectionner les types..." class="w-full" />
                </div>

                <div class="space-y-2">
                    <label class="text-sm font-medium">Dossiers</label>
                    <NSelect v-model:value="modelValue.folders" :options="folders" multiple
                        :loading="modelValue.inLoading" :disabled="modelValue.inLoading" filterable
                        placeholder="Sélectionner les dossiers..." class="w-full" />
                </div>


                <NSpace vertical class="col-span-2 mt-2">
                    <NSwitch v-model:value="modelValue.isDir">
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

                <div class="w-full grid grid-cols-3">
                    <NButton @click="() => {syncTypeFiles(); syncFolders()}" :disabled="modelValue.inLoading" tertiary type="info"
                        class="flex-1">
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
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { NInput, NButton, NIcon, NSelect, type SelectOption, NSwitch, NSpace } from 'naive-ui';
import { Search, SyncCircleOutline, Refresh, Folder, Document } from '@vicons/ionicons5';

const modelValue = defineModel<{
    search: string;
    types: string[];
    folders: string[];
    isDir: boolean;
    inLoading: boolean;
    showPath: boolean;
}>({
    required: true
});

const typeFiles = ref<SelectOption[]>([]);
const folders = ref<SelectOption[]>([]);

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
        const result = await invoke<File[]>('get_all_folders');
        folders.value = result.map(folder => ({
            label: folder.name,
            value: folder.name
        }));
    } catch (error) {
        console.error(error);
    } finally {
        modelValue.value.inLoading = false;
    }
}

</script>