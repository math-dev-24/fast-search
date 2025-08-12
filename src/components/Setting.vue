<script setup lang="ts">
import {ref, onMounted, computed, watch} from 'vue';
import { 
  NButton, NModal, NCard, NDynamicInput, NIcon,
  NInput, NTabs, NTabPane, NAlert, NTag, NForm, NFormItem,
  NText, useMessage
} from 'naive-ui';
import { 
  Settings, FolderOutline, ServerOutline, 
  SaveOutline, RefreshOutline, CheckmarkCircleOutline,
  WarningOutline, CloseCircleOutline
} from '@vicons/ionicons5';
import { useSettingStore, useAiStore } from "../shared";

const showSetting = ref<boolean>(false);
const settingStore = useSettingStore();
const aiStore = useAiStore();
const message = useMessage();

defineProps<{
    inSync: boolean;
}>();

const statusIcon = computed(() => {
    switch (settingStore.status) {
        case 'Ok': return CheckmarkCircleOutline;
        case 'Error': return CloseCircleOutline;
        case 'Loading': return RefreshOutline;
        default: return WarningOutline;
    }
});

const statusType = computed(() => {
    switch (settingStore.status) {
        case 'Ok': return 'success';
        case 'Error': return 'error';
        case 'Loading': return 'info';
        default: return 'warning';
    }
});

onMounted(() => {
    settingStore.init();
    aiStore.checkConnection();
    aiStore.apiUrl = settingStore.ai_path;
});

const handleSaveAll = async () => {
    try {
        if (aiStore.apiUrl !== settingStore.ai_path) {
            aiStore.apiUrl = settingStore.ai_path;
            await aiStore.init();
        }
        await settingStore.savePaths();
        message.success('Paramètres sauvegardés avec succès');
    } catch (error) {
        message.error('Erreur lors de la sauvegarde');
        console.error('Erreur sauvegarde:', error);
    }
};

watch(() => settingStore.ai_path, async (newPath) => {
  aiStore.apiUrl = newPath;
  await aiStore.init();
})

const handleSavePaths = async () => {
    try {
        await settingStore.savePaths();
        message.success('Chemins sauvegardés avec succès');
    } catch (error) {
        message.error('Erreur lors de la sauvegarde des chemins');
    }
};

const handleReset = () => {
    settingStore.resetSettings();
    message.info('Paramètres réinitialisés');
};

</script>

<template>  
    <div>
        <NButton 
            @click="showSetting = !showSetting" 
            tertiary 
            round 
            :disabled="inSync"
            class="transition-all duration-200 hover:scale-105"
        >
            <template #icon>
                <NIcon size="16">
                    <Settings />
                </NIcon>
            </template>
            Réglages
        </NButton>
        
        <NModal v-model:show="showSetting" class="custom-modal">
            <NCard
                title="Paramètres de l'application"
                :bordered="false"
                class="w-full max-w-4xl bg-white dark:bg-gray-900 rounded-xl shadow-2xl"
                :segmented="{
                    content: true,
                    footer: 'soft'
                }"
            >
                <template #header-extra>
                    <NTag :type="statusType" round size="small" class="font-medium">
                        <template #icon>
                            <NIcon size="14">
                                <component :is="statusIcon" />
                            </NIcon>
                        </template>
                        {{ settingStore.status === 'Ok' ? 'Configuré' : settingStore.status === 'Error' ? 'Erreur' : 'Chargement...' }}
                    </NTag>
                </template>

                <!-- Alert de statut -->
                <NAlert 
                    v-if="settingStore.status === 'Error'" 
                    type="error" 
                    class="mb-6" 
                    title="Erreur de configuration"
                >
                    Une erreur s'est produite lors du chargement ou de la sauvegarde des paramètres.
                </NAlert>

                <NTabs type="line" animated class="settings-tabs">
                    <!-- Onglet Chemins de recherche -->
                    <NTabPane name="paths" class="tab-content">
                        <template #tab>
                            <div class="flex items-center gap-2">
                                <NIcon size="18">
                                    <FolderOutline />
                                </NIcon>
                                <span>Chemins de recherche</span>
                                <NTag size="small" round type="info" class="ml-1">
                                    {{ settingStore.paths.length }}
                                </NTag>
                            </div>
                        </template>
                        
                        <div class="space-y-6 p-4">
                            <div>
                                <NText class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3 block">
                                    Dossiers à indexer pour la recherche
                                </NText>
                                <NText depth="3" class="text-xs mb-4 block">
                                    Ajoutez les chemins des dossiers que vous souhaitez inclure dans l'indexation des fichiers.
                                </NText>
                            </div>
                            
                            <NForm>
                                <NFormItem>
                                    <NDynamicInput 
                                        v-model:value="settingStore.paths" 
                                        placeholder="Exemple: C:\Users\Documents ou /home/user/documents"
                                        class="dynamic-input"
                                        :min="0"
                                        :max="20"
                                    />
                                </NFormItem>
                            </NForm>
                            
                            <div class="flex justify-between items-center pt-4 border-t border-gray-200 dark:border-gray-700">
                                <NText class="text-xs text-gray-500 dark:text-gray-400">
                                    {{ settingStore.paths.length }} chemin{{ settingStore.paths.length > 1 ? 's' : '' }} configuré{{ settingStore.paths.length > 1 ? 's' : '' }}
                                </NText>
                                <NButton 
                                    @click="handleSavePaths" 
                                    :loading="settingStore.inLoading"
                                    type="primary"
                                    size="medium"
                                    class="save-button"
                                >
                                    <template #icon>
                                        <NIcon>
                                            <SaveOutline />
                                        </NIcon>
                                    </template>
                                    {{ settingStore.inLoading ? 'Sauvegarde...' : 'Sauvegarder les chemins' }}
                                </NButton>
                            </div>
                        </div>
                    </NTabPane>

                    <!-- Onglet Configuration IA -->
                    <NTabPane name="ai" class="tab-content">
                        <template #tab>
                            <div class="flex items-center gap-1">
                                <NIcon size="18">
                                    <ServerOutline />
                                </NIcon>
                                <span>Service IA</span>
                              <div>
                                <NTag
                                    :type="aiStore.isConnected ? 'success' : 'error'"
                                    size="small"
                                    round
                                    class="ml-1"
                                >
                                  {{ aiStore.isConnected ? 'En ligne' : 'Hors ligne' }}
                                </NTag>
                                <NTag v-if="aiStore.availableModels.length > 0" size="small" round type="info" class="ml-1">
                                  {{aiStore.availableModels.length}}
                                </NTag>
                              </div>
                            </div>
                        </template>
                        
                        <div class="space-y-6 py-2">
                            <div>
                                <NText class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3 block">
                                    Configuration du service d'intelligence artificielle
                                </NText>
                                <NText depth="3" class="text-xs mb-4 block">
                                    Configurez l'URL de votre service LM Studio pour la recherche en langage naturel.
                                </NText>
                            </div>
                            
                            <NForm>
                                <NFormItem label="URL du service" class="ai-url-form-item">
                                    <NInput
                                        v-model:value="settingStore.ai_path"
                                        placeholder="http://localhost:11434 ou http://192.168.1.100:1234"
                                        type="text"
                                        class="ai-url-input"
                                        clearable
                                        :disabled="settingStore.inLoading"
                                    >
                                        <template #prefix>
                                            <NIcon size="16" class="text-gray-400">
                                                <ServerOutline />
                                            </NIcon>
                                        </template>
                                    </NInput>
                                </NFormItem>
                            </NForm>

                            <!-- Informations de connexion -->
                            <NCard>
                                <div class="flex items-center justify-between">
                                    <div>
                                        <NText class="font-medium">Statut de la connexion</NText>
                                        <div class="flex items-center gap-2 mt-1">
                                            <NTag 
                                                :type="aiStore.connectionStatus === 'connected' ? 'success' : 
                                                       aiStore.connectionStatus === 'connecting' ? 'warning' : 'error'"
                                                size="small"
                                                round
                                            >
                                                {{ aiStore.connectionStatus === 'connected' ? 'Connecté' : 
                                                   aiStore.connectionStatus === 'connecting' ? 'Connexion...' : 
                                                   aiStore.connectionStatus === 'error' ? 'Erreur' : 'Déconnecté' }}
                                            </NTag>
                                            <NText depth="3" class="text-xs">
                                                {{ aiStore.availableModels.length }} modèle{{ aiStore.availableModels.length > 1 ? 's' : '' }} disponible{{ aiStore.availableModels.length > 1 ? 's' : '' }}
                                            </NText>
                                        </div>
                                    </div>
                                    <NButton 
                                        @click="aiStore.init"
                                        :loading="aiStore.connectionStatus === 'connecting'"
                                        size="small"
                                        quaternary
                                        class="test-connection-button"
                                    >
                                        <template #icon>
                                            <NIcon>
                                                <RefreshOutline />
                                            </NIcon>
                                        </template>
                                        Tester
                                    </NButton>
                                </div>
                                
                                <div v-if="aiStore.lastError" class="mt-3">
                                    <NText type="error" class="text-xs">
                                        {{ aiStore.lastError }}
                                    </NText>
                                </div>
                            </NCard>
                        </div>
                    </NTabPane>
                </NTabs>

                <template #footer>
                    <div class="flex justify-between items-center gap-4">
                        <NButton 
                            @click="handleReset"
                            quaternary 
                            type="error"
                            size="medium"
                            :disabled="settingStore.inLoading"
                        >
                            <template #icon>
                                <NIcon>
                                    <RefreshOutline />
                                </NIcon>
                            </template>
                            Réinitialiser
                        </NButton>
                        
                        <div class="flex gap-3">
                            <NButton 
                                @click="showSetting = false"
                                quaternary
                                size="medium"
                            >
                                Fermer
                            </NButton>
                            <NButton 
                                @click="handleSaveAll"
                                :loading="settingStore.inLoading"
                                type="primary"
                                size="medium"
                                class="save-all-button"
                            >
                                <template #icon>
                                    <NIcon>
                                        <SaveOutline />
                                    </NIcon>
                                </template>
                                {{ settingStore.inLoading ? 'Sauvegarde...' : 'Tout sauvegarder' }}
                            </NButton>
                        </div>
                    </div>
                </template>
            </NCard>
        </NModal>
    </div>
</template>