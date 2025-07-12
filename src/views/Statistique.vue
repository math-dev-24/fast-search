<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import type { Stat } from '../types';
import { onMounted, ref } from 'vue';
import { NIcon, NSpace, useMessage, NButton, NButtonGroup } from 'naive-ui';
import { FileTrayFull, FolderOpen, CloudUpload, SyncCircle, Refresh, TextOutline } from '@vicons/ionicons5';
import { formatSize } from '../shared/sieFormat';

const stat = ref<Stat | null>(null);
const message = useMessage();
const inLoading = ref<boolean>(false);

onMounted(async () => {
    await getStat();
});


const getStat = async () => {
    inLoading.value = true;
    try {
        const tmp_stat = await invoke<Stat>('get_stat');

        stat.value = tmp_stat;

        message.success('Statistiques récupérées avec succès');

    } catch (error) {

        console.error(error);

        stat.value = {
            nb_files: 0,
            nb_folders: 0,
            total_size: 0,
            indexed_files: 0,
            unindexed_files: 0,
            indexed_percentage: 0.0,
            content_indexed_files: 0,
            uncontent_indexed_files: 0,
            content_indexed_percentage: 0.0
        }

    } finally {
        inLoading.value = false;
    }
}

const resetData = async () => {
    try {
        await invoke('reset_data');
        setTimeout(() => {
            getStat();
        }, 500);
    } catch (error) {
        message.error('Erreur lors de la réinitialisation des données');
        console.error(error);
    }
}

</script>

<template>
    <NSpace v-if="stat" vertical class="py-14 px-2 container mx-auto">
        <NButtonGroup class="my-2">
            <NButton @click="getStat" :loading="inLoading" tertiary type="primary">
                <template #icon>
                    <NIcon>
                        <SyncCircle />
                    </NIcon>
                </template>
                Actualiser
            </NButton>
            <NButton @click="resetData" tertiary type="error">
                <template #icon>
                    <NIcon>
                        <Refresh /> 
                    </NIcon>
                </template>
                Réinitialiser (Suppression des données)
            </NButton>
        </NButtonGroup>

        <NSpace justify="space-around" align="center" class="border rounded-lg px-2 py-10 border-gray-700">
            <NSpace vertical align="center">
                <NIcon size="35" class="text-blue-600">
                    <FileTrayFull />
                </NIcon>
                <div class="text-2xl font-bold text-blue-600">{{ stat.nb_files }}</div>
                <div class="text-sm text-gray-600">Fichiers</div>
            </NSpace>
            <NSpace vertical align="center">
                <NIcon size="35" class="text-stone-500">
                    <FolderOpen />
                </NIcon>
                <div class="text-2xl font-bold text-stone-500">{{ stat.nb_folders }}</div>
                <div class="text-sm text-gray-600">Dossiers</div>
            </NSpace>
            <NSpace vertical align="center">
                <NIcon size="35" class="text-purple-600">
                    <CloudUpload />
                </NIcon>
                <div class="text-2xl font-bold text-purple-600">{{ formatSize(stat.total_size) }}</div>
                <div class="text-sm text-gray-600">Taille totale</div>
            </NSpace>
        </NSpace>
        <NSpace justify="space-around" align="center" class="border rounded-lg px-2 py-10 border-gray-700">
            <NSpace vertical align="center">
                <NIcon size="35" class="text-green-600">
                    <FileTrayFull />
                </NIcon>
                <div class="text-2xl font-bold text-green-600">{{ stat.indexed_files }}</div>
                <div class="text-sm text-gray-600">Fichiers indexés</div>
            </NSpace>
            <NSpace vertical align="center">
                <NIcon size="35" class="text-yellow-600">
                    <FileTrayFull />
                </NIcon>
                <div class="text-2xl font-bold text-yellow-600">{{ stat.unindexed_files }}</div>
                <div class="text-sm text-gray-600">Fichiers non indexés</div>
            </NSpace>
            <NSpace vertical align="center">
                <NIcon size="35" class="text-red-600">
                    <FolderOpen />
                </NIcon>
                <div class="text-2xl font-bold text-red-600">{{ stat.indexed_percentage.toFixed(2) }}%</div>
                <div class="text-sm text-gray-600">Fichiers indexés</div>
            </NSpace>
        </NSpace>
        <NSpace justify="space-around" align="center" class="border rounded-lg px-2 py-10 border-gray-700">
            <NSpace vertical align="center">
                <NIcon size="35" class="text-green-600">
                    <FileTrayFull />
                </NIcon>
                <div class="text-2xl font-bold text-green-600">{{ stat.content_indexed_files }}</div>
                <div class="text-sm text-gray-600">Contenu indexé</div>
            </NSpace>
            <NSpace vertical align="center">
                <NIcon size="35" class="text-yellow-600">
                    <FileTrayFull />
                </NIcon>
                <div class="text-2xl font-bold text-yellow-600">{{ stat.uncontent_indexed_files }}</div>
                <div class="text-sm text-gray-600">Contenu non indexés</div>
            </NSpace>
            <NSpace vertical align="center">
                <NIcon size="35" class="text-red-600">
                    <TextOutline />
                </NIcon>
                <div class="text-2xl font-bold text-red-600">{{ stat.content_indexed_percentage.toFixed(2) }}%</div>
                <div class="text-sm text-gray-600">Contenu indexé</div>
            </NSpace>
        </NSpace>
    </NSpace>
</template>