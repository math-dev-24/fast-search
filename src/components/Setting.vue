<script setup lang="ts">
import { ref } from 'vue';
import { NButton, NModal, NCard, NDynamicInput, NIcon, NSpace } from 'naive-ui';
import { Settings } from '@vicons/ionicons5';
import { useSetting } from '../composables/useSetting';
const { setting } = useSetting();

const showSetting = ref<boolean>(false);

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
                    <p class="text-sm text-gray-400 italic">Chemin du dossier à scanner :</p>
                    <NDynamicInput v-model:value="setting.search_path" placeholder="Chemin du dossier à scanner" />
                </NSpace>
            </NCard>
        </NModal>
    </div>
</template>