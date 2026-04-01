<template>
  <div class="home-view">
    <NLayout>
      <NLayoutContent>
      <div class="mb-4">
        <NTabs v-model:value="modeSearch" size="large" type="line" animated>
          <NTab name="search" :tab="`🔍 ${t('home.tabStandard')}`"/>
          <NTab name="ai_search" :tab="`🤖 ${t('home.tabAi')}`"/>
        </NTabs>
      </div>
      <Search v-if="modeSearch === 'search'" @reset="handleReset" @search="handleSearch"/>
      <SearchWithAI v-if="modeSearch === 'ai_search'" :in-loading="searchStore.in_loading"
                    @search="handleSearchWithAi"/>
      <NDivider/>

      <!-- Skeleton de chargement des résultats -->
      <template v-if="searchStore.in_loading">
        <NSpace vertical size="large">
          <NCard :bordered="false">
            <NSpace vertical>
              <NSkeleton text :repeat="1" width="60%" />
              <NSkeleton text :repeat="3" />
            </NSpace>
          </NCard>
          <NCard>
            <NSpace vertical>
              <NSkeleton text :repeat="1" width="50%" />
              <NSkeleton text :repeat="4" />
            </NSpace>
          </NCard>
        </NSpace>
      </template>

      <!-- Empty state initial -->
      <template v-if="!searchStore.is_loaded && !searchStore.in_loading">
        <NCard class="mt-6">
          <div class="flex flex-col items-center justify-center gap-6 py-16">
            <NEmpty
                class="results-empty"
                :description="t('home.emptyInitialTitle')"
                size="large"
            >
              <template #icon>
                <div class="text-6xl leading-none">🔍</div>
              </template>
              <template #extra>
                <NText class="text-center max-w-md" depth="3">
                  {{ t("home.emptyInitialHint") }}
                </NText>
              </template>
            </NEmpty>
          </div>
        </NCard>
      </template>

      <template v-if="searchStore.is_loaded && !searchStore.in_loading">
        <NCard :bordered="false" class="mb-2" embedded>
          <template #header>
            <NText class="text-lg font-medium">📊 {{ t("home.searchStatsTitle") }}</NText>
          </template>
          <NGrid :cols="3" :x-gap="16">
            <NGi>
              <NStatistic :label="t('home.totalResults')" label-style="color: #666; font-size: 14px;">
                <NNumberAnimation
                    :duration="800"
                    :from="previousTotalResults"
                    :to="searchStore.filterResult.length"
                    class="text-2xl font-bold text-blue-600"
                />
              </NStatistic>
            </NGi>
            <NGi>
              <NStatistic :label="t('home.filesFound')" label-style="color: #666; font-size: 14px;">
                <NNumberAnimation
                    :duration="800"
                    :from="previousFilesCount"
                    :to="searchStore.filterResult.filter(file => !file.is_dir).length"
                    class="text-2xl font-bold text-green-600"
                />
              </NStatistic>
            </NGi>
            <NGi>
              <NStatistic :label="t('home.foldersFound')" label-style="color: #666; font-size: 14px;">
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

        <Transition name="fade-expand" mode="out-in">
          <NSpace v-if="searchStore.filterResult.length > 0" key="results" size="large" vertical>
            <Folders
                :folders="searchStore.filterResult.filter(file => file.is_dir)"
                @openFile="handleOpenFile"
            />
            <Files
                :files="searchStore.filterResult.filter(file => !file.is_dir)"
                @openFile="handleOpenFile"
                @previewFile="handlePreviewFile"
                @copyPath="searchStore.copyPath"
            />
          </NSpace>
          <NCard v-else-if="searchStore.filterResult.length === 0" key="empty" class="mt-6">
            <div class="flex flex-col items-center justify-center gap-6 py-16">
              <NEmpty
                  class="results-empty"
                  :description="t('home.emptyResultsTitle')"
                  size="large"
              >
                <template #icon>
                  <div class="text-6xl leading-none">🔍</div>
                </template>
                <template #extra>
                  <NText class="text-center max-w-md" depth="3">
                    {{ t("home.emptyResultsHint") }}
                  </NText>
                </template>
              </NEmpty>
            </div>
          </NCard>
        </Transition>
      </template>
      </NLayoutContent>
    </NLayout>

    <FileDetail
        :file="detailFile"
        :show="showDetail"
        @update:show="showDetail = false"
    />
  </div>
</template>

<script lang="ts" setup>
import {ref, watch} from 'vue';
import {
  NCard,
  NDivider,
  NEmpty,
  NGi,
  NGrid,
  NLayout,
  NLayoutContent,
  NNumberAnimation,
  NSkeleton,
  NSpace,
  NStatistic,
  NTab,
  NTabs,
  NText,
  useMessage
} from 'naive-ui';
import { useI18n } from "vue-i18n";
import SearchWithAI from '../components/Search/AISearch.vue';
import Folders from '../components/Folders/index.vue';
import Files from '../components/Files/index.vue';
import Filter from '../components/Filter.vue';
import FileDetail from '../components/FileDetail.vue';
import {useSearchStore} from '../shared';
import type {File, SearchQuery} from '../types';
import Search from "../components/Search/Search.vue";

const searchStore = useSearchStore();
const message = useMessage();
const { t } = useI18n();
const modeSearch = ref<string>('search');

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

async function handleOpenFile(path: string) {
  try {
    // Trouver le fichier pour déterminer si c'est un dossier
    const file = searchStore.filterResult.find(f => f.path === path);
    const isDirectory = file?.is_dir ?? false;
    
    if (isDirectory) {
      message.info(t("home.openingFolder"));
    } else {
      message.info(t("home.openingFile"));
    }
    
    await searchStore.openFile(path);
    
    if (isDirectory) {
      message.success(t("home.folderOpened"));
    } else {
      message.success(t("home.fileOpened"));
    }
  } catch (error) {
    console.error('Erreur lors de l\'ouverture:', error);
    message.error(t("home.openError"));
  }
}

async function handleSearch() {
  try {
    await searchStore.searchFiles();
    if (searchStore.result.length === 0) {
      message.info(t("home.noResults"));
    }
  } catch (error) {
    message.error(t("home.searchError"));
    console.error('Erreur recherche:', error);
  }
}

function handleReset() {
  searchStore.reset_search();
  message.info(t("home.searchReset"));
}

async function handleSearchWithAi(query: SearchQuery) {
  try {
    searchStore.query = query;
    modeSearch.value = 'search';
    await searchStore.searchFiles();
    if (searchStore.result.length === 0) {
      message.info(t("home.noResultsAi"));
    }
  } catch (error) {
    message.error(t("home.searchAiError"));
    console.error('Erreur recherche IA:', error);
  }
}

</script>

<style scoped>
.fade-expand-enter-active {
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.fade-expand-leave-active {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.fade-expand-enter-from {
  opacity: 0;
  transform: translateY(20px) scale(0.95);
}

.fade-expand-leave-to {
  opacity: 0;
  transform: translateY(-10px) scale(0.95);
}

:deep(.results-empty .n-empty__icon) {
  margin-bottom: 14px;
}

:deep(.results-empty .n-empty__description) {
  margin-top: 0;
}

:deep(.results-empty .n-empty__extra) {
  margin-top: 14px;
}
</style>
