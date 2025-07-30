<script setup lang="ts">
import { defineModel, onMounted, ref } from 'vue';
import { NInput, NButton, NSpace, NIcon, NTooltip, NSelect, NTag } from 'naive-ui';
import { SearchOutline, RefreshOutline, InformationCircleOutline, CheckmarkCircleOutline, CloseCircleOutline, TimeOutline } from '@vicons/ionicons5';
import type { SearchQuery } from '../types/search';
import { invoke } from '@tauri-apps/api/core';

const naturalSearch = defineModel<string>('naturalSearch', { required: true });
const modelAI = defineModel<string>('modelAI', { required: true });

const listModels = ref<string[]>([]);
const healthCheck = ref<boolean>(false);
const loadingHealthCheck = ref<boolean>(false);

const emit = defineEmits<{
    (e: 'aiSearch', query: string): void;
}>();

const aiSearch = () => {
    emit('aiSearch', naturalSearch.value);
}

const resetSearch = () => {
    naturalSearch.value = '';
}

defineProps<{
    inLoading: boolean;
    isLoaded: boolean;
    query: SearchQuery;
}>()

const getListModels = async () => {
    const models = await invoke<string[]>('ai_list_models');
    listModels.value = models;
    if (models.length > 0) {
        modelAI.value = models[0];
    }
}

const healthCheckAI = async () => {
    loadingHealthCheck.value = true;
    const health = await invoke<boolean>('ai_health_check', { model: modelAI.value });
    healthCheck.value = health;
    loadingHealthCheck.value = false;
    if (health) {
        getListModels();
    }
}

onMounted(() => {
    healthCheckAI();
    getListModels();
});
</script>

<template>
    <div class="search-ai-container">
        <!-- Header redesigné avec statut moderne -->
        <div class="search-header">
            <div class="header-left">
                <h3 class="search-title">Recherche IA</h3>
                <div class="status-indicator">
                    <div class="ai-status">
                        <NTag 
                            :type="healthCheck ? 'success' : loadingHealthCheck ? 'warning' : 'error'"
                            size="small"
                            round
                        >
                            <template #icon>
                                <NIcon size="14">
                                    <CheckmarkCircleOutline v-if="healthCheck && !loadingHealthCheck" />
                                    <TimeOutline v-else-if="loadingHealthCheck" />
                                    <CloseCircleOutline v-else />
                                </NIcon>
                            </template>
                            <span class="status-text">
                                {{ healthCheck && !loadingHealthCheck ? 'Connecté' : loadingHealthCheck ? 'Vérification...' : 'Déconnecté' }}
                            </span>
                        </NTag>
                        
                        <NButton 
                            v-if="!healthCheck && !loadingHealthCheck"
                            @click="healthCheckAI" 
                            size="tiny" 
                            quaternary
                            class="retry-button"
                        >
                            <NIcon size="12">
                                <RefreshOutline />
                            </NIcon>
                        </NButton>
                    </div>
                    
                    <!-- Indicateur de modèle sélectionné -->
                    <div v-if="modelAI" class="model-indicator">
                        <NTag type="info" size="small" round>
                            <span class="model-text">{{ modelAI }}</span>
                        </NTag>
                    </div>
                </div>
            </div>

            <!-- Bouton d'information avec badge -->
            <div class="header-right">
                <NTooltip trigger="hover" placement="top">
                    <template #trigger>
                        <NButton
                            quaternary
                            size="small"
                            class="info-button"
                            :disabled="!isLoaded || !healthCheck"
                        >
                            <NIcon>
                                <InformationCircleOutline />
                            </NIcon>
                        </NButton>
                    </template>
                    <div class="query-info">
                        <h4>Dernière recherche :</h4>
                        <div class="query-details">
                            <p><strong>Terme :</strong> {{ query.text || 'Aucun' }}</p>
                            <p><strong>Filtres :</strong> {{ query.filters ? 'Configurés' : 'Aucun' }}</p>
                            <p><strong>Tri :</strong> {{ query.sort_by }} ({{ query.sort_order }})</p>
                        </div>
                    </div>
                </NTooltip>
            </div>
        </div>

        <div class="search-input-wrapper">
            <NInput
                v-model:value="naturalSearch"
                placeholder="Décrivez ce que vous recherchez..."
                type="textarea"
                :autosize="{ minRows: 3, maxRows: 6 }"
                class="search-textarea"
                clearable
                :disabled="inLoading"
                :loading="inLoading"
            />
        </div>

        <NSelect v-model:value="modelAI" :options="listModels.map(model => ({ label: model, value: model }))" class="mb-4" />
        
        <NSpace justify="center" size="medium" class="action-buttons">
            <NButton
                @click="aiSearch"
                type="primary"
                size="large"
                :disabled="!naturalSearch || inLoading"
                :loading="inLoading"
                class="search-button"
            >
                <template #icon>
                    <NIcon>
                        <SearchOutline />
                    </NIcon>
                </template>
                {{ inLoading ? 'Recherche...' : 'Rechercher' }}
            </NButton>
            
            <NButton
                @click="resetSearch"
                type="default"
                size="large"
                :disabled="!naturalSearch || inLoading"
                class="reset-button"
            >
                <template #icon>
                    <NIcon>
                        <RefreshOutline />
                    </NIcon>
                </template>
                Réinitialiser
            </NButton>
        </NSpace>
    </div>
</template>

<style scoped>
.search-ai-container {
    transition: all 0.3s ease;
}

.search-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 1.5rem;
    padding: 1rem;
    background: var(--n-color-modal);
    border-radius: 12px;
    border: 1px solid var(--n-border-color);
}

.header-left {
    display: flex;
    gap: 0.75rem;
}

.search-title {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 700;
    color: var(--n-text-color);
    background: linear-gradient(135deg, var(--n-primary-color), var(--n-primary-color-hover));
}

.status-indicator {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    flex-wrap: wrap;
}

.ai-status {
    display: flex;
    align-items: center;
    gap: 0.25rem;
}

.status-text {
    font-size: 0.75rem;
    font-weight: 500;
}

.retry-button {
    padding: 2px;
    min-width: 20px;
    height: 20px;
}

.model-indicator {
    display: flex;
    align-items: center;
}

.model-text {
    font-size: 0.75rem;
    font-weight: 500;
}

.header-right {
    display: flex;
    align-items: center;
}

.info-button {
    opacity: 0.7;
    transition: all 0.3s ease;
    border-radius: 8px;
}

.info-button:hover {
    opacity: 1;
    transform: scale(1.05);
}

.info-button:disabled {
    opacity: 0.3;
    cursor: not-allowed;
    transform: none;
}

.query-info {
    max-width: 300px;
    padding: 0.75rem;
}

.query-info h4 {
    margin: 0 0 0.75rem 0;
    font-size: 0.9rem;
    font-weight: 600;
    color: var(--n-text-color);
}

.query-details p {
    margin: 0.25rem 0;
    font-size: 0.85rem;
    color: var(--n-text-color-2);
}

.query-details strong {
    color: var(--n-text-color);
}

.search-input-wrapper {
    margin-bottom: 1.5rem;
}

.search-textarea {
    font-size: 1rem;
    line-height: 1.6;
    border-radius: 8px;
}

.action-buttons {
    margin-top: 1rem;
}

.search-button {
    border-radius: 8px;
    font-weight: 600;
    transition: all 0.3s ease;
}

.search-button:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.reset-button {
    border-radius: 8px;
    font-weight: 500;
    transition: all 0.3s ease;
}

.reset-button:hover {
    transform: translateY(-1px);
}

/* Responsive design */
@media (max-width: 768px) {
    .search-header {
        flex-direction: column;
        gap: 1rem;
        align-items: stretch;
    }
    
    .header-left {
        align-items: center;
    }
    
    .status-indicator {
        justify-content: center;
    }
    
    .header-right {
        justify-content: center;
    }
}
</style>