<script setup lang="ts">
import { useRouter } from 'vue-router';
import { NSpace, NButton, NIcon, NPopover } from 'naive-ui';
import { RouterLink } from 'vue-router';
import Setting from './Setting.vue';
import SyncDetails from './SyncDetails.vue';
import SyncIndicator from './SyncIndicator.vue';
import { useSync } from '../composables/useSync';

const router = useRouter();
const routes = router.getRoutes();

const { 
    inSync, 
    hasError,
    hasSuccess,
    overallProgress, 
    progressStatus, 
    statusIcon, 
    syncSummary,
    processDetails,
    startSync
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
                
                <!-- Bouton de sync avec popover d'info détaillé -->
                <NPopover trigger="hover" :disabled="!inSync && !hasError && !hasSuccess" placement="bottom-end">
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
                    
                    <!-- Utilisation du composant SyncDetails -->
                    <SyncDetails
                        :inSync="inSync"
                        :hasError="hasError.length > 0"
                        :hasSuccess="hasSuccess"
                        :overallProgress="overallProgress"
                        :progressStatus="progressStatus"
                        :syncSummary="syncSummary"
                        :processDetails="processDetails"
                        :statusIcon="statusIcon"
                    />
                </NPopover>

                <Setting :inSync="inSync" />
            </NSpace>
        </NSpace>
        
        <!-- Indicateur de synchronisation -->
        <SyncIndicator
            :inSync="inSync"
            :hasError="hasError.length > 0"
            :hasSuccess="hasSuccess"
            :overallProgress="overallProgress"
            :progressStatus="progressStatus"
            :syncSummary="syncSummary"
        />
    </header>
</template>