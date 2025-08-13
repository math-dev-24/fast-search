<script setup lang="ts">
import { NCard, NIcon, NInputNumber, NSpace } from 'naive-ui';
import { Resize } from '@vicons/ionicons5';
import { useSearchStore } from '../../shared';

const emit = defineEmits<{ (e: 'handleSearch'): void }>();
const searchStore = useSearchStore();

const handleSizeChange = () => {
  if (searchStore.auto_submit) emit('handleSearch');
};
</script>

<template>
  <NCard class="filter-card" size="small">
    <template #header>
      <div class="filter-header">
        <NIcon class="text-purple-500" size="16">
          <Resize />
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
          @update-value="handleSizeChange"
        />
      </div>
    </NSpace>
  </NCard>
</template>
