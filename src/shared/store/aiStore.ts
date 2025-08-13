import {defineStore} from 'pinia';
import {invoke} from '@tauri-apps/api/core';
import type {SearchQuery} from '../../types';

interface AiState {
    isConnected: boolean;
    inLoading: boolean;
    isLoaded: boolean;
    naturalSearch: string;
    availableModels: string[];
    selectedModel: string;
    connectionStatus: 'connected' | 'connecting' | 'disconnected' | 'error';
    lastError: string | null;
    apiUrl: string;
}

export const useAiStore = defineStore('ai', {
    state: (): AiState => ({
        isConnected: false,
        naturalSearch: '',
        inLoading: false,
        isLoaded: false,
        availableModels: [],
        selectedModel: 'llama3.2',
        connectionStatus: 'disconnected',
        lastError: null,
        apiUrl: 'http://localhost:11434',
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
        async checkConnection(): Promise<boolean> {
            this.connectionStatus = 'connecting';
            this.lastError = null;

            try {
                const result = await invoke<boolean>('ai_health_check', { aiUrl: this.apiUrl});
                
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
                const models = await invoke<string[]>('ai_list_models', {aiUrl: this.apiUrl});
                this.availableModels = models;

                if (models.length > 0) {
                    this.selectedModel = models[0] as string
                }
            } catch (error) {
                this.lastError = error instanceof Error ? error.message : 'Erreur lors du chargement des modèles';
                console.error('Erreur lors de la récupération des modèles:', error);
            }
        },

        async aiSearch(): Promise<SearchQuery | undefined> {
            this.inLoading = true;
            this.isLoaded = false;
            try {
                return await invoke<SearchQuery>('ai_search', {
                    aiUrl: this.apiUrl,
                    naturalQuery: this.naturalSearch,
                    model: this.selectedModel
                });
            } catch (error) {
                console.error(error);
            } finally {
                this.inLoading = false;
            }
        },

        async init() {
            await this.checkConnection();
            await this.loadModels();
        }
    }
})