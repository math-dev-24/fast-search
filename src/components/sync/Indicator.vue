<script setup lang="ts">
import { NProgress, NText, NFlex, NIcon } from 'naive-ui';
import { SyncCircle, CheckmarkCircle, AlertCircle } from '@vicons/ionicons5';
import type { ProgressStatus } from 'naive-ui';

interface Props {
  inSync: boolean;
  hasError: boolean;
  hasSuccess: boolean;
  overallProgress: number;
  progressStatus: ProgressStatus;
  syncSummary: string;
}

defineProps<Props>();

const getStatusIcon = (hasError: boolean, hasSuccess: boolean, inSync: boolean) => {
  if (hasError) return AlertCircle;
  if (hasSuccess && !inSync) return CheckmarkCircle;
  return SyncCircle;
};
</script>

<template>
  <div v-if="inSync || hasError || hasSuccess" class="sync-indicator">
    <NProgress 
      type="line" 
      :percentage="overallProgress" 
      :show-indicator="false" 
      :show-text="false" 
      :height="3"
      :status="progressStatus"
    />
    
    <!-- Message d'état visible -->
    <div class="px-8 py-2 text-sm">
      <NFlex align="center" justify="space-between">
        <NFlex align="center" class="flex-1">
          <NIcon size="14" class="mr-2">
            <component :is="getStatusIcon(hasError, hasSuccess, inSync)" />
          </NIcon>
          <NText :type="progressStatus" class="truncate">
            {{ syncSummary }}
          </NText>
        </NFlex>
        <NText v-if="inSync" depth="3" class="text-xs ml-4">
          {{ Math.round(overallProgress) }}%
        </NText>
      </NFlex>
    </div>
  </div>
</template>

<style scoped>
.sync-indicator {
  position: relative;
}
</style> 