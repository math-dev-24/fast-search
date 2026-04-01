<script setup lang="ts">
import {ref, computed, watch} from 'vue';
import { useI18n } from "vue-i18n";
import { 
  NButton, NModal, NCard, NDynamicInput, NIcon,
  NInput, NTabs, NTabPane, NAlert, NTag, NForm, NFormItem,
  NText, NSelect, useMessage
} from 'naive-ui';
import { 
  Settings, FolderOutline, ServerOutline, 
  SaveOutline, RefreshOutline, CheckmarkCircleOutline,
  WarningOutline, CloseCircleOutline
} from '@vicons/ionicons5';
import { useSettingStore, useAiStore } from "../shared";

const showSetting = ref<boolean>(false);
const activeTab = ref<'paths' | 'ai'>('paths');
const settingsInitialized = ref<boolean>(false);
const aiInitialized = ref<boolean>(false);
const apiKeyInput = ref<string>('');
const settingStore = useSettingStore();
const aiStore = useAiStore();
const message = useMessage();
const { t } = useI18n();

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

const ensureSettingsInitialized = async () => {
    if (settingsInitialized.value) return;
    await settingStore.init();
    aiStore.syncFromSettings();
    if (settingStore.ai_default_model) {
        aiStore.selectedModel = settingStore.ai_default_model;
    }
    settingsInitialized.value = true;
};

const ensureAiInitialized = async () => {
    if (aiInitialized.value) return;
    await aiStore.init();
    aiInitialized.value = true;
};

const toggleSettingsModal = async () => {
    if (!showSetting.value) {
        await ensureSettingsInitialized();
    }
    showSetting.value = !showSetting.value;
};

const handleSaveAll = async () => {
    try {
        settingStore.ai_default_model = aiStore.selectedModel;
        await settingStore.savePaths();
        settingStore.saveAiSettings();
        await aiStore.init();
        message.success(t("messages.settingsSaved"));
    } catch (error) {
        message.error(t("messages.settingsSaveError"));
        console.error('Erreur sauvegarde:', error);
    }
};

const providerOptions = [
    { label: 'LM Studio (local)', value: 'lm_studio' },
    { label: 'Ollama (local)', value: 'ollama' },
    { label: 'ChatGPT / OpenAI', value: 'open_ai' },
    { label: 'Claude / Anthropic', value: 'anthropic' },
    { label: 'Mistral', value: 'mistral' },
];

const needsApiKey = computed(() => aiStore.requiresApiKey(settingStore.ai_provider));
const isLocalProvider = computed(() =>
    settingStore.ai_provider === 'lm_studio' || settingStore.ai_provider === 'ollama'
);

watch(
    () => settingStore.ai_provider,
    async (provider) => {
        if (provider === 'lm_studio' && !settingStore.ai_endpoint.startsWith('http://localhost:1234')) {
            settingStore.ai_endpoint = 'http://localhost:1234';
        }
        if (provider === 'ollama' && !settingStore.ai_endpoint.startsWith('http://localhost:11434')) {
            settingStore.ai_endpoint = 'http://localhost:11434';
        }
        if (provider === 'open_ai') settingStore.ai_endpoint = 'https://api.openai.com';
        if (provider === 'anthropic') settingStore.ai_endpoint = 'https://api.anthropic.com';
        if (provider === 'mistral') settingStore.ai_endpoint = 'https://api.mistral.ai';

        aiStore.selectedProvider = provider;
        aiStore.apiUrl = settingStore.ai_endpoint;
        await aiStore.refreshApiKeyStatus();
        await aiStore.loadModels();
        settingStore.ai_default_model = aiStore.selectedModel;
    },
);

watch(
    () => settingStore.ai_endpoint,
    async (endpoint) => {
        aiStore.apiUrl = endpoint;
        await aiStore.loadModels();
        settingStore.ai_default_model = aiStore.selectedModel;
    },
);

watch(
    [showSetting, activeTab],
    async ([isOpen, tab]) => {
        if (!isOpen) return;
        if (tab === 'ai') {
            await ensureAiInitialized();
        }
    },
);

const handleSaveApiKey = async () => {
    if (!apiKeyInput.value.trim()) {
        message.warning(t("messages.enterApiKey"));
        return;
    }
    try {
        aiStore.selectedProvider = settingStore.ai_provider;
        await aiStore.saveApiKey(apiKeyInput.value.trim());
        apiKeyInput.value = '';
        message.success(t("messages.apiKeySaved"));
    } catch (error) {
        message.error(t("messages.apiKeySaveError"));
    }
};

const handleDeleteApiKey = async () => {
    try {
        aiStore.selectedProvider = settingStore.ai_provider;
        await aiStore.deleteApiKey();
        message.success(t("messages.apiKeyDeleted"));
    } catch (error) {
        message.error(t("messages.apiKeyDeleteError"));
    }
};

const handleSavePaths = async () => {
    try {
        await settingStore.savePaths();
        message.success(t("messages.pathsSaved"));
    } catch (error) {
        message.error(t("messages.pathsSaveError"));
    }
};

const handleReset = () => {
    settingStore.resetSettings();
    aiStore.syncFromSettings();
    message.info(t("messages.settingsReset"));
};

const handleTestProvider = async () => {
    aiStore.selectedProvider = settingStore.ai_provider;
    aiStore.apiUrl = settingStore.ai_endpoint;
    await aiStore.refreshApiKeyStatus();
    await aiStore.checkConnection();
    await aiStore.loadModels();
    settingStore.ai_default_model = aiStore.selectedModel;
};

</script>

<template>  
    <div>
        <NButton 
            @click="toggleSettingsModal"
            tertiary 
            round 
            :disabled="inSync"
        >
            <template #icon>
                <NIcon size="16">
                    <Settings />
                </NIcon>
            </template>
            {{ t("header.settings") }}
        </NButton>
        
        <NModal v-model:show="showSetting" class="custom-modal">
            <NCard
                :title="t('settings.title')"
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
                        {{ settingStore.status === 'Ok' ? t("settings.configured") : settingStore.status === 'Error' ? t("settings.errorStatus") : t("common.loading") }}
                    </NTag>
                </template>

                <!-- Alert de statut -->
                <NAlert 
                    v-if="settingStore.status === 'Error'" 
                    type="error" 
                    class="mb-6" 
                    :title="t('settings.configurationErrorTitle')"
                >
                    {{ t("settings.configurationErrorText") }}
                </NAlert>

                <NTabs v-model:value="activeTab" type="line" animated class="settings-tabs">
                    <!-- Onglet Chemins de recherche -->
                    <NTabPane name="paths" class="tab-content">
                        <template #tab>
                            <div class="flex items-center gap-2">
                                <NIcon size="18">
                                    <FolderOutline />
                                </NIcon>
                                <span>{{ t("settings.pathsTab") }}</span>
                                <NTag size="small" round type="info" class="ml-1">
                                    {{ settingStore.paths.length }}
                                </NTag>
                            </div>
                        </template>
                        
                        <div class="space-y-6 p-4">
                            <div>
                                <NText class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3 block">
                                    {{ t("settings.foldersToIndex") }}
                                </NText>
                                <NText depth="3" class="text-xs mb-4 block">
                                    {{ t("settings.foldersToIndexHint") }}
                                </NText>
                            </div>
                            
                            <NForm>
                                <NFormItem>
                                    <NDynamicInput 
                                        v-model:value="settingStore.paths" 
                                        :placeholder="t('settings.pathsPlaceholder')"
                                        class="dynamic-input"
                                        :min="0"
                                        :max="20"
                                    />
                                </NFormItem>
                            </NForm>
                            
                            <div class="flex justify-between items-center pt-4 border-t border-gray-200 dark:border-gray-700">
                                <NText class="text-xs text-gray-500 dark:text-gray-400">
                                    {{ t("settings.pathsConfigured", { count: settingStore.paths.length, suffix: settingStore.paths.length > 1 ? "s" : "" }) }}
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
                                    {{ settingStore.inLoading ? t("settings.saving") : t("settings.savePaths") }}
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
                                <span>{{ t("settings.aiTab") }}</span>
                              <div>
                                <NTag
                                    :type="aiStore.isConnected ? 'success' : 'error'"
                                    size="small"
                                    round
                                    class="ml-1"
                                >
                                  {{ aiStore.isConnected ? t("settings.online") : t("settings.offline") }}
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
                                    {{ t("settings.aiConfigTitle") }}
                                </NText>
                                <NText depth="3" class="text-xs mb-4 block">
                                    {{ t("settings.aiConfigHint") }}
                                </NText>
                            </div>
                            
                            <NForm>
                                <NFormItem :label="t('settings.providerLabel')">
                                    <NSelect
                                        v-model:value="settingStore.ai_provider"
                                        :options="providerOptions"
                                        :placeholder="t('settings.providerPlaceholder')"
                                    />
                                </NFormItem>

                                <NFormItem v-if="isLocalProvider" :label="t('settings.localUrl')" class="ai-url-form-item">
                                    <NInput
                                        v-model:value="settingStore.ai_endpoint"
                                        :placeholder="t('settings.localUrlPlaceholder')"
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

                                <NFormItem v-else :label="t('settings.cloudEndpoint')">
                                    <NInput v-model:value="settingStore.ai_endpoint" disabled />
                                </NFormItem>

                                <NFormItem :label="t('settings.defaultModel')">
                                    <NSelect
                                        v-model:value="aiStore.selectedModel"
                                        :options="aiStore.availableModelOptions"
                                        :placeholder="t('settings.defaultModelPlaceholder')"
                                    />
                                </NFormItem>
                            </NForm>

                            <NCard v-if="needsApiKey">
                                <div class="space-y-3">
                                    <NText class="font-medium">{{ t("settings.apiKeyTitle") }}</NText>
                                    <NInput
                                        v-model:value="apiKeyInput"
                                        type="password"
                                        show-password-on="mousedown"
                                        :placeholder="t('settings.apiKeyPlaceholder')"
                                    />
                                    <div class="flex items-center gap-2">
                                        <NTag :type="aiStore.hasApiKey ? 'success' : 'warning'" round size="small">
                                            {{ aiStore.hasApiKey ? t("settings.apiKeyConfigured") : t("settings.apiKeyMissing") }}
                                        </NTag>
                                        <NButton size="small" type="primary" @click="handleSaveApiKey">{{ t("settings.saveApiKey") }}</NButton>
                                        <NButton size="small" quaternary type="error" @click="handleDeleteApiKey">{{ t("settings.deleteApiKey") }}</NButton>
                                    </div>
                                </div>
                            </NCard>

                            <!-- Informations de connexion -->
                            <NCard>
                                <div class="flex items-center justify-between">
                                    <div>
                                        <NText class="font-medium">{{ t("settings.connectionStatus") }}</NText>
                                        <div class="flex items-center gap-2 mt-1">
                                            <NTag 
                                                :type="aiStore.connectionStatus === 'connected' ? 'success' : 
                                                       aiStore.connectionStatus === 'connecting' ? 'warning' : 'error'"
                                                size="small"
                                                round
                                            >
                                                {{ aiStore.connectionStatus === 'connected' ? t("aiSearch.connected") :
                                                   aiStore.connectionStatus === 'connecting' ? t("aiSearch.connecting") :
                                                   aiStore.connectionStatus === 'error' ? t("aiSearch.error") : t("aiSearch.disconnected") }}
                                            </NTag>
                                            <NText depth="3" class="text-xs">
                                                {{ t("settings.modelsAvailable", { count: aiStore.availableModels.length, suffix: aiStore.availableModels.length > 1 ? "s" : "" }) }}
                                            </NText>
                                        </div>
                                    </div>
                                    <NButton 
                                        @click="handleTestProvider"
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
                                        {{ t("settings.test") }}
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
                            {{ t("common.reset") }}
                        </NButton>
                        
                        <div class="flex gap-3">
                            <NButton 
                                @click="showSetting = false"
                                quaternary
                                size="medium"
                            >
                                {{ t("settings.close") }}
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
                                {{ settingStore.inLoading ? t("settings.saving") : t("settings.saveAll") }}
                            </NButton>
                        </div>
                    </div>
                </template>
            </NCard>
        </NModal>
    </div>
</template>