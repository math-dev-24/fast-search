<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { NButton, NModal, NCard, NDynamicInput, NIcon, NSpace } from 'naive-ui';
import { Settings } from '@vicons/ionicons5';
import { invoke } from '@tauri-apps/api/core';

const showSetting = ref<boolean>(false);

const paths = ref<string[]>([]);
const isLoading = ref<boolean>(false);

defineProps<{
    inSync: boolean;
}>();

onMounted(async () => {
    try {
        isLoading.value = true;
        paths.value = await invoke<string[]>('get_all_paths');
    } catch (error) {
        console.error(error);
    } finally {
        isLoading.value = false;
    }
});

const savePaths = async () => {
    try {
        isLoading.value = true;
        await invoke('save_paths', { paths: paths.value });
    } catch (error) {
        console.error(error);
    } finally {
        isLoading.value = false;
    }
};

</script>

<template>  
    <div>
        <NButton @click="showSetting = !showSetting" tertiary round :disabled="inSync">
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
                    <p class="text-sm text-gray-400 italic">{{ paths.length }} Chemin{{ paths.length > 1 ? 's' : '' }} du dossier{{ paths.length > 1 ? 's' : '' }} à scanner :</p>
                    <NDynamicInput v-model:value="paths" placeholder="Chemin du dossier à scanner" />
                    <NButton @click="savePaths" :loading="isLoading">{{ isLoading ? 'Sauvegarde en cours...' : 'Sauvegarder' }}</NButton>
                </NSpace>
            </NCard>
        </NModal>
    </div>
</template>