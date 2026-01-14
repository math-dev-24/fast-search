<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import type { Stat } from '../types';
import { onMounted, onUnmounted, ref } from 'vue';
import { NIcon, NSpace, useMessage, NButton, NButtonGroup, NNumberAnimation } from 'naive-ui';
import { FileTrayFull, FolderOpen, CloudUpload, SyncCircle, Refresh, TextOutline } from '@vicons/ionicons5';

const defaultStat: Stat = {
    nb_files: 0,
    nb_folders: 0,
    total_size: 0,
    indexed_files: 0,
    unindexed_files: 0,
    indexed_percentage: 0.0,
    content_indexed_files: 0,
    uncontent_indexed_files: 0,
    content_indexed_percentage: 0.0,
    unindexable_files: 0
}

const stat = ref<Stat>(defaultStat);
const previousStat = ref<Stat>(defaultStat);
const unit = ref<string>('Mo');

const message = useMessage();
const inLoading = ref<boolean>(false);

const listeners: UnlistenFn[] = [];

onMounted(async () => {
    await getStat();
});

onMounted(async () => {
    listeners.push(await listen('stat_updated', (event: any) => {
        previousStat.value = { ...stat.value };
        stat.value = event.payload;
    })); 
});

onUnmounted(() => {
    listeners.forEach(listener => listener());
});

const getStat = async () => {
    inLoading.value = true;
    try {
        const tmp_stat = await invoke<Stat>('get_stat');
        stat.value = tmp_stat;
        if (message) {
            message.success('Statistiques actualisées avec succès');
        }
    } catch (error) {
        stat.value = defaultStat;
        if (message) {
            message.error('Erreur lors de la récupération des statistiques');
        }
        console.error(error);
    } finally {
        inLoading.value = false;
    }
}

const resetData = async () => {
    // Utiliser window.$dialog si disponible, sinon créer une confirmation simple
    const confirmed = window.confirm(
        '⚠️ Attention : Cette action va supprimer toutes les données indexées.\n\n' +
        'Cela inclut :\n' +
        '- Tous les fichiers et dossiers indexés\n' +
        '- Tout le contenu indexé\n' +
        '- Toutes les statistiques\n\n' +
        'Cette action est irréversible. Êtes-vous sûr de vouloir continuer ?'
    );
    
    if (!confirmed) {
        return;
    }
    
    try {
        if (message) {
            message.info('Réinitialisation des données en cours...');
        }
        await invoke('reset_data');
        if (message) {
            message.success('Données réinitialisées avec succès');
        }
        setTimeout(() => {
            getStat();
        }, 500);
    } catch (error) {
        if (message) {
            message.error('Erreur lors de la réinitialisation des données');
        }
        console.error(error);
    }
}

const octetToAdapted = (value: number) => {
    const Mp = value / 1024 / 1024;
    const Go = Mp / 1024;
    const To = Go / 1024;

    if (To > 1) {
        unit.value = 'To';
        return To;
    } else if (Go > 1) {
        unit.value = 'Go';
        return Go;
    } else if (Mp > 1) {
        unit.value = 'Mo';
        return Mp;
    } else {
        unit.value = 'o';
        return value;
    }
}

</script>

<template>
    <div class="statistique-view">
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
            <NButton @click="resetData" tertiary type="error" class="danger-button">
                <template #icon>
                    <NIcon>
                        <Refresh /> 
                    </NIcon>
                </template>
                Réinitialiser les données
            </NButton>
        </NButtonGroup>

        <NSpace justify="space-around" align="center" class="border rounded-lg px-2 py-10 border-gray-700">
            <NSpace vertical align="center">
                <NIcon size="35" class="text-blue-600">
                    <FileTrayFull />
                </NIcon>
                <div class="text-xl font-bold flex items-center text-blue-600">
                    <NNumberAnimation :from="previousStat.nb_files" :to="stat.nb_files" :duration="1000" />
                </div>
                <div class="text-sm text-gray-600">Fichiers</div>
            </NSpace>
            <NSpace vertical align="center">
                <NIcon size="35" class="text-stone-500">
                    <FolderOpen />
                </NIcon>
                <div class="text-xl font-bold flex items-center text-stone-500">
                    <NNumberAnimation :from="previousStat.nb_folders" :to="stat.nb_folders" :duration="1000" />
                </div>
                <div class="text-sm text-gray-600">Dossiers</div>
            </NSpace>
            <NSpace vertical align="center">
                <NIcon size="35" class="text-purple-600">
                    <CloudUpload />
                </NIcon>
                <div class="text-xl font-bold flex items-center text-purple-600">
                    <NNumberAnimation :from="octetToAdapted(previousStat.total_size)" :to="octetToAdapted(stat.total_size)" :duration="1000" />
                    <span class="text-sm ml-1">{{ unit }}</span>
                </div>
                <div class="text-sm text-gray-600">Taille totale</div>
            </NSpace>
        </NSpace>
        <NSpace justify="space-around" align="center" class="border rounded-lg px-2 py-10 border-gray-700">
            <NSpace vertical align="center">
                <NIcon size="35" class="text-green-600">
                    <FileTrayFull />
                </NIcon>
                <div class="text-xl font-bold flex items-center text-green-600">
                    <NNumberAnimation :from="previousStat.indexed_files" :to="stat.indexed_files" :duration="1000" />
                </div>
                <div class="text-sm text-gray-600">Fichiers indexés</div>
            </NSpace>
            <NSpace vertical align="center">
                <NIcon size="35" class="text-yellow-600">
                    <FileTrayFull />
                </NIcon>
                <div class="text-xl font-bold flex items-center text-yellow-600">
                    <NNumberAnimation :from="previousStat.unindexed_files" :to="stat.unindexed_files" :duration="1000" />
                </div>
                <div class="text-sm text-gray-600">Fichiers non indexés</div>
            </NSpace>
            <NSpace vertical align="center">
                <NIcon size="35" class="text-red-600">
                    <FolderOpen />
                </NIcon>
                <div class="text-xl font-bold flex items-center text-red-600">
                    <NNumberAnimation :from="previousStat.indexed_percentage" :to="stat.indexed_percentage" :duration="1000" />
                    %
                </div>
                <div class="text-sm text-gray-600">Fichiers indexés</div>
            </NSpace>
        </NSpace>
        <NSpace justify="space-around" align="center" class="border rounded-lg px-2 py-10 border-gray-700">
            <NSpace vertical align="center">
                <NIcon size="35" class="text-green-600">
                    <FileTrayFull />
                </NIcon>
                <div class="text-xl font-bold flex items-center text-green-600">
                    <NNumberAnimation :from="previousStat.content_indexed_files" :to="stat.content_indexed_files" :duration="1000" />
                </div>
                <div class="text-sm text-gray-600">Contenu indexé</div>
            </NSpace>
            <NSpace vertical align="center">
                <NIcon size="35" class="text-yellow-600">
                    <FileTrayFull />
                </NIcon>
                <div class="text-xl font-bold flex items-center text-yellow-600">
                    <NNumberAnimation :from="previousStat.uncontent_indexed_files" :to="stat.uncontent_indexed_files" :duration="1000" />
                </div>
                <div class="text-sm text-gray-600">Contenu non indexés</div>
            </NSpace>
            <NSpace vertical align="center">
                <NIcon size="35" class="text-red-600">
                    <FileTrayFull />
                </NIcon>
                <div class="text-xl font-bold flex items-center text-red-600">
                    <NNumberAnimation :from="previousStat.unindexable_files" :to="stat.unindexable_files" :duration="1000" />
                </div>
                <div class="text-sm text-gray-600">Fichiers non indexables</div>
            </NSpace>
            <NSpace vertical align="center">
                <NIcon size="35" class="text-red-600">
                    <TextOutline />
                </NIcon>
                <div class="text-xl font-bold flex items-center text-red-600">
                    <NNumberAnimation :from="previousStat.content_indexed_percentage" :to="stat.content_indexed_percentage" :duration="1000" />
                    %
                </div>
                <div class="text-sm text-gray-600">Contenu indexé</div>
            </NSpace>
        </NSpace>
        </NSpace>
    </div>
</template>