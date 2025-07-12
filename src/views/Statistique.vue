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

const message = useMessage();
const inLoading = ref<boolean>(false);

const listeners: UnlistenFn[] = [];

onMounted(async () => {
    await getStat();
});

onMounted(async () => {
    listeners.push(await listen('stat_updated', (event: any) => {
        previousStat.value = stat.value;
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
    } catch (error) {
        stat.value = defaultStat;
        console.error(error);
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

const octetToMo = (value: number) => {
    return value / 1024 / 1024;
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
                    <NNumberAnimation :from="octetToMo(previousStat.total_size)" :to="octetToMo(stat.total_size)" :duration="1000" />
                    <span class="text-sm ml-1">Mo</span>
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
</template>