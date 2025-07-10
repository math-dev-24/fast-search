<script setup lang="ts">
import { useRouter } from 'vue-router';
import { NSpace, NButton, NIcon, NProgress, NPopover, NText, NAlert } from 'naive-ui';
import { RouterLink } from 'vue-router';
import Setting from './Setting.vue';
import { useSync } from '../composables/useSync';

const router = useRouter();
const routes = router.getRoutes();

const { 
    inSync, 
    syncMessage, 
    syncError, 
    syncSuccess, 
    startSync, 
    valueProgress, 
    progressStatus, 
    statusIcon, 
    progressText 
} = useSync();
</script>

<template>
    <header>
        <NSpace justify="space-between" align="center" class="px-8 py-4">
            <h1 class="text-2xl font-bold">
                Fast Search
            </h1>
            <NSpace align="center">
                <RouterLink v-for="route in routes" :key="route.path" :to="route.path" custom
                    v-slot="{ navigate, isActive }">
                    <NButton :class="{ 'active': isActive }" @click="navigate">
                        {{ route.name }}
                    </NButton>
                </RouterLink>
                
                <!-- Bouton de sync avec popover d'info -->
                <NPopover trigger="hover" :disabled="!inSync && !syncError && !syncSuccess">
                    <template #trigger>
                        <NButton 
                            @click="startSync" 
                            :loading="inSync" 
                            tertiary 
                            round 
                            :type="progressStatus"
                            :disabled="inSync"
                        >
                            <template #icon>
                                <NIcon size="16">
                                    <component :is="statusIcon" />
                                </NIcon>
                            </template>
                            Sync
                        </NButton>
                    </template>
                    <div class="max-w-xs">
                        <NText v-if="syncError" type="error">
                            {{ syncError }}
                        </NText>
                        <NText v-else-if="syncSuccess" type="success">
                            {{ syncMessage }}
                        </NText>
                        <div v-else-if="inSync">
                            <NText>{{ progressText }}</NText>
                            <div class="mt-2">
                                <NText depth="3" class="text-xs">
                                    {{ Math.round(valueProgress) }}% terminé
                                </NText>
                            </div>
                        </div>
                    </div>
                </NPopover>

                <Setting :inSync="inSync" />
            </NSpace>
        </NSpace>
        
        <!-- Barre de progression avec couleur selon l'état -->
        <NProgress 
            v-show="inSync || syncError || syncSuccess" 
            type="line" 
            :percentage="valueProgress" 
            :show-indicator="false" 
            :show-text="false" 
            :height="2"
            :status="progressStatus"
        />
        
        <!-- Message d'état visible -->
        <div v-if="(inSync || syncError || syncSuccess) && progressText" 
             class="px-8 py-1 text-xs text-gray-600 bg-gray-800 border-b">
            <NText :type="progressStatus" class="truncate">
                {{ progressText }}
            </NText>
        </div>
        
        <!-- Alerte d'erreur -->
        <NAlert 
            v-if="syncError" 
            type="error" 
            :show-icon="true"
            closable
            @close="syncError = ''"
            class="mx-8 mt-2"
        >
            {{ syncError }}
        </NAlert>
    </header>
</template>