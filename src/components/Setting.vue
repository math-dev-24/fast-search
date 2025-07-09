<script setup lang="ts">
import { ref } from 'vue';
import { NButton, NModal, NCard, NInput, NIcon, NSpace, useMessage } from 'naive-ui';
import { Settings, SyncCircle } from '@vicons/ionicons5';
import { invoke } from '@tauri-apps/api/core';
import { useSetting } from '../composables/useSetting';

const { setting } = useSetting();

const message = useMessage();

const showSetting = ref<boolean>(false);
 const inLoading = ref<boolean>(false);


const syncFilesAndFolders = async () => {
    inLoading.value = true;
    try {
        const tmp_path = setting.search_path.replace(/\\/g, "/");

        await invoke('sync_files_and_folders', { path: tmp_path });

        message.success('Synchronisation des fichiers et des dossiers terminée');
    } catch (error) {
        message.error('Erreur lors de la synchronisation des fichiers et des dossiers');
    } finally {
        inLoading.value = false;
    }
}

</script>

<template>  
    <div>
        <NButton @click="showSetting = !showSetting" tertiary round>
            <template #icon>
                <NIcon size="16">
                    <Settings />
                </NIcon>
            </template>
            Réglages
        </NButton>
        <NModal v-model:show="showSetting">
            <NCard
                title="Paramètres"
                :bordered="false"
                class="max-w-xl"
            >
                <NSpace vertical>
                    <NInput v-model:value="setting.search_path" placeholder="Chemin du dossier à scanner" />
                    <NSpace justify="end" align="center">
                        <NButton @click="syncFilesAndFolders" :loading="inLoading" :disabled="inLoading">
                            <template #icon>
                                <NIcon size="16">
                                    <SyncCircle />
                                </NIcon>
                            </template>
                            Sync
                        </NButton>
                    </NSpace>
                </NSpace>
            </NCard>
        </NModal>
    </div>
</template>