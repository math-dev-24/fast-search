<template>
  <NLayout class="min-h-screen">
    <NLayoutHeader class="pt-2 px-6">
      <NTabs
          v-model:value="modeSearch"
          size="large"
          type="line"
      >
        <NTab name="search" tab="🔍 Recherche Standard"/>
        <NTab name="ai_search" tab="🤖 Recherche IA"/>
      </NTabs>
    </NLayoutHeader>

    <NLayoutContent class="p-6">
      <Search v-if="modeSearch === 'search'" @reset="searchStore.reset_search" @search="searchStore.searchFiles"/>
      <SearchWithAI v-if="modeSearch === 'ai_search'" :in-loading="searchStore.in_loading" @search="handleSearchWithAi"/>
      <NDivider/>

      <template v-if="searchStore.is_loaded">
        <NCard :bordered="false" class="mb-2" embedded>
          <template #header>
            <NText class="text-lg font-medium">📊 Statistiques de recherche</NText>
          </template>
          <NGrid :cols="3" :x-gap="16">
            <NGi>
              <NStatistic label="Total des résultats" label-style="color: #666; font-size: 14px;">
                <NNumberAnimation
                    :duration="800"
                    :from="previousTotalResults"
                    :to="searchStore.filterResult.length"
                    class="text-2xl font-bold text-blue-600"
                />
              </NStatistic>
            </NGi>
            <NGi>
              <NStatistic label="Fichiers trouvés" label-style="color: #666; font-size: 14px;">
                <NNumberAnimation
                    :duration="800"
                    :from="previousFilesCount"
                    :to="searchStore.filterResult.filter(file => !file.is_dir).length"
                    class="text-2xl font-bold text-green-600"
                />
              </NStatistic>
            </NGi>
            <NGi>
              <NStatistic label="Dossiers trouvés" label-style="color: #666; font-size: 14px;">
                <NNumberAnimation
                    :duration="800"
                    :from="previousFoldersCount"
                    :to="searchStore.filterResult.filter(file => file.is_dir).length"
                    class="text-2xl font-bold text-purple-600"
                />
              </NStatistic>
            </NGi>
          </NGrid>
        </NCard>

        <Filter v-model="searchStore"/>


        <NSpace v-if="searchStore.filterResult.length > 0" size="large" vertical>
          <Folder  />

          <div v-if="searchStore.filterResult.filter(file => !file.is_dir).length > 0">
            <NCard :bordered="false" embedded title="📄 Fichiers">
              <template #header-extra>
                <NBadge
                    :max="999"
                    :value="searchStore.filterResult.filter(file => !file.is_dir).length"
                    type="success"
                />
              </template>
              <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                <CardFile
                    v-for="file in searchStore.filterResult.filter(f => !f.is_dir).slice(0, maxFiles)"
                    :key="file.name"
                    :file="file"
                    @copyPath="searchStore.copyPath"
                    @openFile="handleOpenFile"
                    @previewFile="handlePreviewFile"
                />
              </div>
              <div v-if="searchStore.filterResult.filter(f => !f.is_dir).length > maxFiles"
                   class="mt-6 text-center">
                <NButton
                    class="w-full max-w-md"
                    secondary
                    size="large"
                    type="primary"
                    @click="maxFiles += 50"
                >
                  📄 Voir {{ Math.min(50, searchStore.filterResult.filter(f => !f.is_dir).length - maxFiles) }}
                  fichiers de plus
                </NButton>
              </div>
            </NCard>
          </div>
        </NSpace>

        <NCard v-else class="mt-6">
          <div class="flex flex-col items-center justify-center py-16">
            <NEmpty
                class="mb-4"
                description="Aucun résultat trouvé"
                size="large"
            >
              <template #icon>
                <div class="text-6xl">🔍</div>
              </template>
              <template #extra>
                <NText class="text-center max-w-md" depth="3">
                  Essayez de modifier vos critères de recherche ou utilisez des mots-clés différents
                </NText>
              </template>
            </NEmpty>
          </div>
        </NCard>
      </template>
    </NLayoutContent>
  </NLayout>

  <FileDetail
      :file="detailFile"
      :show="showDetail"
      @update:show="showDetail = false"
  />

</template>

<script lang="ts" setup>
import {ref, watch} from 'vue';
import {
  NBadge,
  NButton,
  NCard,
  NDivider,
  NEmpty,
  NGi,
  NGrid,
  NLayout,
  NLayoutContent,
  NLayoutHeader,
  NNumberAnimation,
  NSpace,
  NStatistic,
  NTab,
  NTabs,
  NText,
  useMessage
} from 'naive-ui';
import SearchWithAI from '../components/Search/AISearch.vue';
import Folder from '../components/Folder/index.vue';
import Filter from '../components/Filter.vue';
import FileDetail from '../components/FileDetail.vue';
import {useSearchStore} from '../shared';
import type {File, SearchQuery} from '../types';
import Search from "../components/Search/Search.vue";
import CardFile from "../components/File/CardFile.vue";


const searchStore = useSearchStore();
const modeSearch = ref<string>('search');
const maxFiles = ref<number>(20);

const detailFile = ref(null as null | File);
const showDetail = ref(false);

const previousTotalResults = ref<number>(0);
const previousFilesCount = ref<number>(0);
const previousFoldersCount = ref<number>(0);

watch(() => searchStore.filterResult, () => {
  previousTotalResults.value = searchStore.filterResult.length;
  previousFilesCount.value = searchStore.filterResult.filter(file => !file.is_dir).length;
  previousFoldersCount.value = searchStore.filterResult.filter(file => file.is_dir).length;
}, {deep: true});

function handlePreviewFile(file: File) {
  detailFile.value = file;
  showDetail.value = true;
}

function handleOpenFile(path: string) {
  const message = useMessage();
  message.success('Le dossier va être ouvert (cela peu prendre quelque secondes)');
  searchStore.openFile(path);
}

async function handleSearchWithAi(query: SearchQuery) {
  searchStore.query = query;
  modeSearch.value = 'search';
  await searchStore.searchFiles();
}

</script>

