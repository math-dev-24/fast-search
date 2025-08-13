<script setup lang="ts">
import { NDivider, NIcon, NSwitch } from 'naive-ui';
import { Document, DocumentText, Folder, Refresh, RefreshCircle } from '@vicons/ionicons5';
import { useSearchStore } from '../../shared';

const emit = defineEmits<{ (e: 'handleSearch'): void }>();
const searchStore = useSearchStore();

const handleIsDirChange = () => {
  if (searchStore.auto_submit) emit('handleSearch');
};
</script>

<template>
  <NDivider class="options-divider">Options de recherche</NDivider>
  <div class="search-options grid grid-cols-3">
    <div class="option-group">
      <NSwitch v-model:value="searchStore.query.filters.is_dir" size="medium" @update:value="handleIsDirChange">
        <template #checked>
          <NIcon class="text-blue-500" size="16">
            <Folder />
          </NIcon>
        </template>
        <template #unchecked>
          <NIcon class="text-gray-400" size="16">
            <Document />
          </NIcon>
        </template>
      </NSwitch>
      <div class="option-label">
        <span class="option-title">{{ searchStore.query.filters.is_dir ? 'Dossiers uniquement' : 'Tous les fichiers' }}</span>
        <span class="option-desc">{{ searchStore.query.filters.is_dir ? 'Rechercher dans les dossiers seulement' : 'Inclure fichiers et dossiers' }}</span>
      </div>
    </div>

    <div class="option-group">
      <NSwitch v-model:value="searchStore.auto_submit" size="medium">
        <template #checked>
          <NIcon class="text-green-500" size="16">
            <Refresh />
          </NIcon>
        </template>
        <template #unchecked>
          <NIcon class="text-gray-400" size="16">
            <RefreshCircle />
          </NIcon>
        </template>
      </NSwitch>
      <div class="option-label">
        <span class="option-title">{{ searchStore.auto_submit ? 'Recherche automatique' : 'Recherche manuelle' }}</span>
        <span class="option-desc">{{ searchStore.auto_submit ? 'Résultats en temps réel' : 'Recherche sur validation' }}</span>
      </div>
    </div>

    <div class="option-group">
      <NSwitch v-model:value="searchStore.query.filters.search_in_content" size="medium">
        <template #checked>
          <NIcon class="text-purple-500" size="16">
            <DocumentText />
          </NIcon>
        </template>
        <template #unchecked>
          <NIcon class="text-gray-400" size="16">
            <Document />
          </NIcon>
        </template>
      </NSwitch>
      <div class="option-label">
        <span class="option-title">{{ searchStore.query.filters.search_in_content ? 'Recherche dans le contenu' : 'Recherche dans les métadonnées' }}</span>
        <span class="option-desc">{{ searchStore.query.filters.search_in_content ? 'Analyser le contenu des fichiers' : 'Nom et propriétés seulement' }}</span>
      </div>
    </div>
  </div>
</template>
