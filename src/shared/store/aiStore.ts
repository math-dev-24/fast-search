import {defineStore} from 'pinia';
import {invoke} from '@tauri-apps/api/core';
import type { AiProvider, AiProviderConfig, SearchQuery } from '../../types';
import { useSettingStore } from "./settingStore";

interface AiState {
    isConnected: boolean;
    inLoading: boolean;
    isLoaded: boolean;
    naturalSearch: string;
    availableModels: string[];
    selectedModel: string;
    selectedProvider: AiProvider;
    connectionStatus: 'connected' | 'connecting' | 'disconnected' | 'error';
    lastError: string | null;
    apiUrl: string;
    hasApiKey: boolean;
}

export const useAiStore = defineStore('ai', {
    state: (): AiState => ({
        isConnected: false,
        naturalSearch: '',
        inLoading: false,
        isLoaded: false,
        availableModels: [],
        selectedModel: 'llama3.2',
        selectedProvider: 'lm_studio',
        connectionStatus: 'disconnected',
        lastError: null,
        apiUrl: 'http://localhost:1234',
        hasApiKey: false,
    }),

    getters: {
        isOperational(): boolean {
            return this.isConnected && this.connectionStatus === 'connected';
        },

        availableModelOptions(): { label: string; value: string }[] {
            return this.availableModels
                .map(model => ({
                    label: model,
                    value: model
                }));
        }
    },

    actions: {
        currentConfig(): AiProviderConfig {
            return {
                provider: this.selectedProvider,
                endpoint: this.apiUrl,
                model: this.selectedModel || null,
                credential_ref: this.hasApiKey ? { credential_id: this.selectedProvider } : null,
            };
        },

        syncFromSettings() {
            const settingStore = useSettingStore();
            this.selectedProvider = settingStore.ai_provider;
            this.apiUrl = settingStore.ai_endpoint;
            this.selectedModel = settingStore.ai_default_model || this.selectedModel;
        },

        requiresApiKey(provider: AiProvider): boolean {
            return ['open_ai', 'anthropic', 'mistral'].includes(provider);
        },

        async refreshApiKeyStatus(): Promise<void> {
            if (!this.requiresApiKey(this.selectedProvider)) {
                this.hasApiKey = false;
                return;
            }
            try {
                this.hasApiKey = await invoke<boolean>('ai_has_api_key', {
                    provider: this.selectedProvider,
                });
            } catch (error) {
                this.hasApiKey = false;
                this.lastError = error instanceof Error ? error.message : 'Erreur sur le statut de la clé API';
            }
        },

        async saveApiKey(apiKey: string): Promise<void> {
            await invoke('ai_save_api_key', { provider: this.selectedProvider, apiKey });
            this.hasApiKey = true;
        },

        async deleteApiKey(): Promise<void> {
            await invoke('ai_delete_api_key', { provider: this.selectedProvider });
            this.hasApiKey = false;
        },

        async checkConnection(): Promise<boolean> {
            this.connectionStatus = 'connecting';
            this.lastError = null;

            try {
                const result = await invoke<boolean>('ai_health_check', { config: this.currentConfig() });
                
                this.isConnected = result;
                this.connectionStatus = result ? 'connected' : 'disconnected';
                return result;
            } catch (error) {
                this.isConnected = false;
                this.connectionStatus = 'error';
                this.lastError = error instanceof Error ? error.message : 'Erreur de connexion inconnue';
                console.error('Erreur lors de la vérification de la connexion AI:', error);
                return false;
            }
        },
        async loadModels(): Promise<void> {
            try {
                const models = await invoke<string[]>('ai_list_models', { config: this.currentConfig() });
                this.availableModels = models;

                if (models.length === 0) {
                    this.selectedModel = '';
                    return;
                }

                if (!models.includes(this.selectedModel)) {
                    this.selectedModel = models[0] as string
                }
            } catch (error) {
                this.lastError = error instanceof Error ? error.message : 'Erreur lors du chargement des modèles';
                console.error('Erreur lors de la récupération des modèles:', error);
                this.availableModels = [];
                this.selectedModel = '';
            }
        },

        async aiSearch(): Promise<SearchQuery | undefined> {
            this.inLoading = true;
            this.isLoaded = false;
            try {
                return await invoke<SearchQuery>('ai_search', {
                    naturalQuery: this.naturalSearch,
                    config: this.currentConfig(),
                });
            } catch (error) {
                console.error(error);
            } finally {
                this.inLoading = false;
            }
        },

        async init() {
            this.syncFromSettings();
            await this.refreshApiKeyStatus();
            await this.checkConnection();
            await this.loadModels();
        }
    }
})