<template>
    <NSpace vertical class="pt-14 px-6">
        <Search v-model="searchStore" @search="searchStore.searchFiles" @reset="searchStore.reset_search" />
        <NDivider />
        <template v-if="searchStore.isLoaded">
            <NCard class="mb-4">
                <NSpace justify="space-around" align="center">
                    <NStatistic label="Résultats">
                        <NNumberAnimation :from="0" :to="searchStore.filterResult.length" :duration="1000" />
                    </NStatistic>
                    <NDivider vertical />
                    <NStatistic label="Fichiers">
                        <NNumberAnimation :from="0" :to="searchStore.filterResult.filter(file => !file.is_dir).length" :duration="1000" />
                    </NStatistic>
                    <NDivider vertical />
                    <NStatistic label="Dossiers">
                        <NNumberAnimation :from="0" :to="searchStore.filterResult.filter(file => file.is_dir).length" :duration="1000" />
                    </NStatistic>
                </NSpace>
            </NCard>
            <Filter v-model="searchStore" />
            <div v-if="searchStore.filterResult.length > 0" class="mt-4">
                <div v-if="searchStore.filterResult.filter(file => file.is_dir).length > 0" class="mb-8">
                    <NText class="text-xs font-semibold uppercase tracking-wider text-gray-500 mb-2" depth="3">
                        Dossiers ({{ searchStore.filterResult.filter(file => file.is_dir).length }})
                    </NText>
                    <NGrid :cols="4" :x-gap="4" :y-gap="4">
                        <NGi v-for="file in searchStore.filterResult.filter(file => file.is_dir).slice(0, maxFolders)" :key="file.name">
                            <CardFolder 
                                :file="file" 
                                @openFile="searchStore.openFile" 
                                :showPath="searchStore.showPath" 
                            />
                        </NGi>
                    </NGrid>
                    <div v-if="searchStore.filterResult.filter(file => file.is_dir).length > maxFolders" class="flex justify-center items-center mt-6 w-full">
                        <NButton @click="maxFolders += 10" tertiary type="info" class="w-full">
                            Voir plus ({{ searchStore.filterResult.filter(file => file.is_dir).length - maxFolders }})
                        </NButton>
                    </div>
                </div>
                
                <div v-if="searchStore.filterResult.filter(file => !file.is_dir).length > 0" class="mb-8">
                    <NText class="text-xs font-semibold uppercase tracking-wider text-gray-500 mb-2" depth="3">
                        Fichiers ({{ searchStore.filterResult.filter(file => !file.is_dir).length }})
                    </NText>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
                        <CardFile 
                            v-for="file in searchStore.filterResult.filter(file => !file.is_dir).slice(0, maxFiles)"
                            :key="file.name" 
                            :file="file"
                            @openFile="searchStore.openFile" 
                            @copyPath="searchStore.copyPath"
                            @previewFile="handlePreviewFile"
                            :showPath="searchStore.showPath"
                        />
                    </div>
                    <div v-if="searchStore.filterResult.filter(file => !file.is_dir).length > maxFiles" class="flex justify-center items-center mt-6 w-full">
                            <NButton @click="maxFiles += 50" tertiary type="info" class="w-full">
                                Voir plus ({{ searchStore.filterResult.filter(file => !file.is_dir).length - maxFiles }})
                            </NButton>
                        </div>
                </div>
            </div>
            
            <div v-else class="flex justify-center items-center min-h-[200px]">
                <NEmpty description="Aucun résultat trouvé" />
            </div>
        </template>
    </NSpace>
    <FilePreview
        :show="showPreview"
        :file="previewFile" 
        @update:show="showPreview = false"
    />
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { NSpace, NGrid, NGi, NDivider, NStatistic, NCard, NText, NEmpty, NButton, NNumberAnimation } from 'naive-ui';
import Search from '../components/Search.vue';
import CardFile from '../components/CardFile.vue';
import CardFolder from '../components/CardFolder.vue';
import Filter from '../components/Filter.vue';
import FilePreview from '../components/FilePreview.vue';
import { useSearchStore } from '../shared/store/search';
import type { File } from '../types';

const searchStore = useSearchStore();
const maxFiles = ref<number>(20);
const maxFolders = ref<number>(8);

const previewFile = ref(null as null | File);
const showPreview = ref(false);

function handlePreviewFile(file: File) {
    previewFile.value = file;
    showPreview.value = true;
}

</script>

