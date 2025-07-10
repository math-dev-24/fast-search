<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { NSpace, NButton, NIcon, useMessage } from 'naive-ui';
import { RouterLink } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { SyncCircle } from '@vicons/ionicons5';
import Setting from './Setting.vue';
import { useSetting } from '../composables/useSetting';

const router = useRouter();
const routes = router.getRoutes();
const message = useMessage();
const { setting } = useSetting();

const isSyncing = ref(false);

const startSync = async () => {
  if (setting.search_path.length === 0) {
    message.warning('Aucun chemin configuré. Veuillez configurer les chemins dans les paramètres.');
    return;
  }
  
  isSyncing.value = true;
  
  try {
    console.log(setting.search_path);
    await invoke('sync_files_and_folders', { paths: setting.search_path });
    message.success('Synchronisation terminée');
  } catch (error) {
    console.error(error);
    message.error('Erreur lors de la synchronisation');
  } finally {
    isSyncing.value = false;
  }
};
</script>

<template>
    <header>
        <NSpace justify="space-between" align="center" class="px-8 py-4">
            <h1 class="text-2xl font-bold">
                Fast Search
            </h1>
            <NSpace align="center">
                <RouterLink v-for="route in routes" :key="route.path" :to="route.path" custom v-slot="{ navigate, isActive }">
                    <NButton :class="{ 'active': isActive }" @click="navigate">
                        {{ route.name }}
                    </NButton>
                </RouterLink>
                
                <!-- Bouton de synchronisation ou indicateur -->
                <div v-if="!isSyncing" class="flex items-center">
                    <NButton @click="startSync" type="primary" size="small">
                        <template #icon>
                            <NIcon size="16">
                                <SyncCircle />
                            </NIcon>
                        </template>
                        Sync
                    </NButton>
                </div>
                
                <div v-else class="flex items-center space-x-2">
                    <div class="flex items-center text-sm text-blue-700">
                        <NIcon size="16" class="animate-spin mr-2">
                            <SyncCircle />
                        </NIcon>
                        <span>Synchronisation en cours...</span>
                    </div>
                </div>
                <Setting />
            </NSpace>
        </NSpace>
    </header>
</template>