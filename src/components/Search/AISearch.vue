<script lang="ts" setup>
import {computed, onMounted} from 'vue';
import {NAlert, NButton, NIcon, NInput, NSelect, NSpace, NTag, NText} from 'naive-ui';
import { useI18n } from "vue-i18n";
import {
  CheckmarkCircleOutline,
  CloseCircleOutline,
  RefreshOutline,
  SearchOutline,
  TimeOutline,
} from '@vicons/ionicons5';
import {useAiStore} from "../../shared";
import {SearchQuery} from "../../types";


const aiStore = useAiStore();
const { t } = useI18n();

const props = defineProps<{
  inLoading: boolean;
}>()

const emit = defineEmits<{
  (e: 'search', query: SearchQuery): void;
}>();

const connectionStatusIcon = computed(() => {
  switch (aiStore.connectionStatus) {
    case 'connected':
      return CheckmarkCircleOutline;
    case 'connecting':
      return TimeOutline;
    case 'error':
      return CloseCircleOutline;
    default:
      return CloseCircleOutline;
  }
});

const connectionStatusType = computed(() => {
  switch (aiStore.connectionStatus) {
    case 'connected':
      return 'success';
    case 'connecting':
      return 'warning';
    case 'error':
      return 'error';
    default:
      return 'default';
  }
});

const connectionStatusText = computed(() => {
  switch (aiStore.connectionStatus) {
    case 'connected':
      return t("aiSearch.connected");
    case 'connecting':
      return t("aiSearch.connecting");
    case 'error':
      return t("aiSearch.error");
    default:
      return t("aiSearch.disconnected");
  }
});

const getButtonLabel = () => {
  if (aiStore.inLoading || props.inLoading) {
    return t("aiSearch.searching");
  }
  if (aiStore.connectionStatus !== 'connected') {
    return t("aiSearch.waitingConnection");
  }
  if (!aiStore.selectedModel) {
    return t("aiSearch.selectModel");
  }
  if (!aiStore.naturalSearch?.trim()) {
    return t("aiSearch.enterQuery");
  }
  return t("aiSearch.search");
};

const handleSearch = async () => {
  const query: SearchQuery | undefined = await aiStore.aiSearch()
  console.info(query);
  if (!query) {
    // Afficher un message si la recherche IA n'a pas retourné de query valide
    return;
  }
  emit('search', query)
}

onMounted(() => {
  aiStore.init()
})

</script>

<template>
  <div class="p-6 transition-all duration-300">
    <div class="search-header flex justify-between items-start mb-6 p-4">
      <div class="header-left flex items-center gap-3">
        <div class="flex items-center gap-2">
          <div class="w-2 h-2 bg-gradient-to-r from-blue-500 to-indigo-500 rounded-full animate-pulse"></div>
            <h3 class="search-title text-xl font-bold">{{ t("aiSearch.title") }}</h3>
        </div>
        <div class="status-indicator flex items-center gap-2">
          <div class="ai-status flex items-center gap-1">
            <NTag
                :type="connectionStatusType"
                class="font-medium shadow-sm transition-all duration-200"
                round
                size="small"
            >
              <template #icon>
                <NIcon size="14">
                  <component :is="connectionStatusIcon"/>
                </NIcon>
              </template>
              <span class="status-text text-xs font-semibold">
                                {{ connectionStatusText }}
                            </span>
            </NTag>
            <NTag v-if="aiStore.selectedModel" class="font-medium shadow-sm" round size="small" type="info">
              <span class="model-text text-xs font-semibold">{{ aiStore.selectedModel }}</span>
            </NTag>

            <NButton
                v-if="aiStore.connectionStatus !== 'connected' && aiStore.connectionStatus !== 'connecting'"
                class="retry-button ml-1 hover:bg-blue-50 dark:hover:bg-gray-700 transition-all duration-200"
                quaternary
                size="tiny"
                @click="aiStore.checkConnection()"
            >
              <NIcon size="12">
                <RefreshOutline/>
              </NIcon>
            </NButton>
          </div>
        </div>
      </div>
      <NText depth="3" class="text-xs italic">
        {{ aiStore.selectedProvider }} | {{aiStore.apiUrl}}
        <NButton text size="tiny" @click="() => {}" class="ml-1">
          ({{ t("aiSearch.configure") }})
        </NButton>
      </NText>
    </div>

    <div class="search-input-wrapper mb-6">
      <div class="relative">
        <NInput
            v-model:value="aiStore.naturalSearch"
            :autosize="{ minRows: 3, maxRows: 6 }"
            :disabled="aiStore.connectionStatus !== 'connected' || !aiStore.selectedModel"
            class="search-textarea transition-all duration-200"
            clearable
            :placeholder="t('aiSearch.placeholder')"
            type="textarea"
        />
        <div class="absolute bottom-2 right-2 text-xs text-gray-400 dark:text-gray-500 font-medium">
          {{ aiStore.naturalSearch?.length || 0 }} {{ t("aiSearch.characters") }}
        </div>
      </div>
    </div>

    <div class="mb-6">
      <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
        {{ t("aiSearch.modelLabel") }}
      </label>
      <NSelect
          v-model:value="aiStore.selectedModel"
          :disabled="aiStore.connectionStatus !== 'connected' || aiStore.availableModels.length === 0"
          :options="aiStore.availableModelOptions"
          class="w-full transition-all duration-200"
          :placeholder="t('aiSearch.modelPlaceholder')"
      />
    </div>

    <NSpace class="action-buttons mt-8" justify="center" size="medium">
      <NButton
          :disabled="!aiStore.naturalSearch?.trim() || aiStore.connectionStatus !== 'connected' || !aiStore.selectedModel || aiStore.inLoading || props.inLoading"
          size="large"
          type="primary"
          @click="handleSearch"
      >
        <template #icon>
          <NIcon class="mr-1">
            <SearchOutline/>
          </NIcon>
        </template>
        {{ getButtonLabel() }}
      </NButton>
    </NSpace>

    <div v-if="aiStore.connectionStatus !== 'connected' && aiStore.connectionStatus !== 'connecting'" class="mt-6">
      <NAlert
          :show-icon="true"
          type="warning"
      >
        <template #header>
          {{ t("aiSearch.serviceUnavailable") }}
        </template>
        <div class="space-y-2">
          <NText>
            {{ t("aiSearch.serviceUnavailableHint") }}
          </NText>
          <ul class="list-disc list-inside space-y-1 ml-2">
            <li>{{ t("aiSearch.hintProviderAvailable") }}</li>
            <li>{{ t("aiSearch.hintApiKey") }}</li>
            <li>{{ t("aiSearch.hintEndpoint") }}</li>
            <li>{{ t("aiSearch.hintDefaultUrl") }} <code class="text-xs bg-gray-100 dark:bg-gray-800 px-1 rounded">http://localhost:1234</code></li>
            <li>{{ t("aiSearch.hintSettings") }}</li>
          </ul>
        </div>
      </NAlert>
    </div>

    <div v-if="aiStore.connectionStatus === 'connected' && !aiStore.selectedModel" class="mt-6">
      <NAlert
          :show-icon="true"
          type="info"
      >
        {{ t("aiSearch.noModelSelected") }}
      </NAlert>
    </div>
  </div>
</template>