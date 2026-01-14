<script setup lang="ts">
import { NButton, NButtonGroup, NDivider, NIcon, NPopover } from 'naive-ui';
import { Refresh, SearchOutline } from '@vicons/ionicons5';
import { useSearchStore } from '../../shared';

const emit = defineEmits<{ (e: 'reset'): void; (e: 'search'): void }>();
const searchStore = useSearchStore();
</script>

<template>
  <NDivider class="actions-divider" />
  <div class="search-actions">
    <NButtonGroup>
      <NButton 
        v-if="!searchStore.auto_submit"
        type="primary" 
        @click="emit('search')"
      >
        <template #icon>
          <NIcon>
            <SearchOutline />
          </NIcon>
        </template>
        Appliquer les filtres
      </NButton>
      <NPopover trigger="hover" placement="top">
        <template #trigger>
          <NButton type="default" @click="emit('reset')">
            <template #icon>
              <NIcon>
                <Refresh />
              </NIcon>
            </template>
            Réinitialiser
          </NButton>
        </template>
        <span>Remet à zéro tous les filtres et la recherche</span>
      </NPopover>
    </NButtonGroup>
  </div>
</template>
