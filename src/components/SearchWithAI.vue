<script setup lang="ts">
import { defineModel } from 'vue';
import { NInput, NButton, NSpace, NIcon, NTooltip } from 'naive-ui';
import { SearchOutline, RefreshOutline, InformationCircleOutline } from '@vicons/ionicons5';
import type { SearchQuery } from '../types/search';

const naturalSearch = defineModel<string>('naturalSearch', { required: true });

const aiSearch = () => {
    console.log(naturalSearch.value);
}

const resetSearch = () => {
    naturalSearch.value = '';
}

defineProps<{
    inLoading: boolean;
    isLoaded: boolean;
    query: SearchQuery;
}>()

</script>

<template>
    <div class="search-ai-container">
        <div class="search-header">
            <h3 class="search-title">Recherche IA</h3>
            <NTooltip trigger="hover" placement="top">
                <template #trigger>
                    <NButton
                        quaternary
                        size="small"
                        class="info-button"
                        :disabled="!isLoaded"
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
    align-items: center;
    margin-bottom: 1rem;
}

.search-title {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--n-text-color);
}

.info-button {
    opacity: 0.7;
    transition: opacity 0.3s ease;
}

.info-button:hover {
    opacity: 1;
}

.info-button:disabled {
    opacity: 0.3;
    cursor: not-allowed;
}

.query-info {
    max-width: 300px;
    padding: 0.5rem;
}

.query-info h4 {
    margin: 0 0 0.5rem 0;
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
}

.search-textarea :deep(.n-input__textarea-el) {
    border-radius: 8px;
    border: 2px solid var(--n-border-color);
    transition: border-color 0.3s ease;
}

.search-textarea :deep(.n-input__textarea-el:focus) {
    border-color: var(--n-primary-color);
    box-shadow: 0 0 0 3px rgba(var(--n-primary-color-rgb), 0.1);
}

.search-textarea :deep(.n-input__textarea-el:disabled) {
    background-color: var(--n-color-disabled);
    cursor: not-allowed;
}

.action-buttons {
    width: 100%;
}

.search-button {
    min-width: 140px;
    font-weight: 600;
    border-radius: 8px;
    transition: all 0.3s ease;
}

.search-button:hover:not(:disabled) {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(var(--n-primary-color-rgb), 0.3);
}

.search-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none;
}



</style>