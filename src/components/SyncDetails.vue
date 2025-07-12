<script setup lang="ts">
import { NCard, NProgress, NText, NIcon, NFlex, NAlert, NSpace } from 'naive-ui';
import {AlertCircle, CheckmarkCircle } from '@vicons/ionicons5';

interface ProcessDetail {
  name: string;
  icon: any;    
  isActive: boolean;
  progress: number;
  message: string;
  currentPath: string;
  phase: string;
  error: string;
  success: boolean;
}

interface Props {
  inSync: boolean;
  hasError: boolean;
  hasSuccess: boolean;
  overallProgress: number;
  progressStatus: 'info' | 'success' | 'error' | 'warning';
  syncSummary: string;
  processDetails: ProcessDetail[];
  statusIcon: any;
}

defineProps<Props>();
</script>

<template>
  <div class="sync-details">
    <!-- Résumé de la synchronisation -->
    <NText class="font-medium mb-3 block">
      {{ syncSummary }}
    </NText>
    
    <!-- Progression globale -->
    <div v-if="inSync" class="mb-4">
      <NFlex align="center" justify="space-between" class="mb-1">
        <NText class="text-sm">Progression globale</NText>
        <NText depth="3" class="text-xs">
          {{ Math.round(overallProgress) }}%
        </NText>
      </NFlex>
      <NProgress 
        type="line" 
        :percentage="overallProgress" 
        :show-indicator="false"
        :height="6"
        :status="progressStatus"
      />
    </div>
    
    <!-- Détails des processus -->
    <div v-if="processDetails.length > 0" class="space-y-3">
      <div v-for="process in processDetails" :key="process.name" class="process-item">
        <NCard size="small" :bordered="false" class="bg-gray-50 dark:bg-gray-800">
          <NFlex align="center" class="mb-2">
            <NIcon size="16" class="mr-2">
              <component :is="process.icon" />
            </NIcon>
            <NText class="text-sm font-medium">{{ process.name }}</NText>
            <NText v-if="process.isActive" depth="3" class="text-xs ml-auto">
              {{ Math.round(process.progress) }}%
            </NText>
            <NIcon v-if="process.success" size="14" class="ml-2 text-green-500">
              <CheckmarkCircle />
            </NIcon>
            <NIcon v-if="process.error && process.error.length > 0" size="14" class="ml-2 text-red-500">
              <AlertCircle />
            </NIcon>
          </NFlex>
          
          <!-- Progression du processus -->
          <NProgress 
            v-if="process.isActive || process.success"
            type="line" 
            :percentage="process.progress" 
            :show-indicator="false"
            :height="3"
            :status="process.success ? 'success' : (process.error && process.error.length > 0 ? 'error' : 'info')"
            class="mb-2"
          />
          
          <!-- Informations du processus -->
          <NSpace vertical size="small">
            <!-- Message du processus -->
            <NText v-if="process.message" depth="3" class="text-xs block truncate">
              {{ process.message }}
            </NText>
            
            <!-- Chemin en cours -->
            <NText v-if="process.currentPath" depth="3" class="text-xs block truncate">
              Fichier: {{ process.currentPath.split('/').pop() || process.currentPath.split('\\').pop() }}
            </NText>
            
            <!-- Phase du processus -->
            <NText v-if="process.phase && process.phase !== 'collecting'" depth="3" class="text-xs block">
              Phase: {{ process.phase }}
            </NText>
            
            <!-- Erreur du processus -->
            <NAlert v-if="process.error && process.error.length > 0" type="error" size="small" class="mt-2">
              {{ process.error }}
            </NAlert>
          </NSpace>
        </NCard>
      </div>
    </div>
    
    <!-- Messages d'état -->
    <div class="mt-4">
      <!-- Message d'erreur global -->
      <NAlert v-if="hasError" type="error">
        <template #icon>
          <NIcon>
            <component :is="statusIcon" />
          </NIcon>
        </template>
        Erreur de synchronisation
      </NAlert>
      
      <!-- Message de succès -->
      <NAlert v-if="hasSuccess && !inSync" type="success">
        <template #icon>
          <NIcon>
            <component :is="statusIcon" />
          </NIcon>
        </template>
        Synchronisation terminée avec succès
      </NAlert>
    </div>
  </div>
</template>

<style scoped>
.sync-details {
  min-width: 320px;
  max-width: 480px;
}

.process-item {
  transition: all 0.2s ease;
}

.process-item:hover {
  transform: translateY(-1px);
}
</style> 