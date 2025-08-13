<script setup lang="ts">
import { NButton, NButtonGroup, NDivider, NIcon } from 'naive-ui';
import { Refresh } from '@vicons/ionicons5';

const emit = defineEmits<{ (e: 'reset'): void }>();
</script>

<template>
  <NDivider class="actions-divider" />
  <div class="search-actions">
    <NButtonGroup>
      <NButton type="default" @click="emit('reset')">
        <template #icon>
          <NIcon>
            <Refresh />
          </NIcon>
        </template>
        Réinitialiser
      </NButton>
    </NButtonGroup>
  </div>
</template>
