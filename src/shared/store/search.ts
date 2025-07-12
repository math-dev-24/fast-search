import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import type { File } from '../../types/file';
import { DateTime } from 'luxon';


type SearchState = {
    search: string;
    searchResult: string;
    types: string[];
    folders: string[];
    sizeLimit: [number, number];
    dateRange: [number, number];
    dateMode: 'create' | 'modify';
    isDir: boolean;
    inLoading: boolean;
    result: File[];
    isLoaded: boolean;
    showPath: boolean;
    searchInPath: boolean;
    autoSubmit: boolean;
    searchInContent: boolean;
    contentSearchLimit: number;
}

export const useSearchStore = defineStore('search', {
    state: (): SearchState => ({
        search: '',
        searchResult: '',
        types: [],
        folders: [],
        sizeLimit: [0, 1000],
        dateRange: [0, DateTime.now().endOf('day').toMillis()],
        dateMode: 'modify',
        isDir: false,
        inLoading: false,
        isLoaded: false,
        showPath: true,
        searchInPath: false,
        result: [],
        autoSubmit: true,
        searchInContent: false,
        contentSearchLimit: 1000
    }),

    getters: {
        filterResult(): File[] {
            return this.result.filter(
                file => file.name.toLowerCase().includes(this.searchResult.toLowerCase())
            );
        },

        options(): { label: string; value: string }[] {
            const allNames = this.result.map(file => file.name);
            const uniqueNames = [...new Set(allNames)];
            
            const filteredSuggestions = uniqueNames.filter(name => 
                name.toLowerCase().includes(this.searchResult.toLowerCase())
            );
            
            return filteredSuggestions.slice(0, 5).map(name => ({
                label: name,
                value: name
            }));
        }
    },

    actions: {
        async searchFiles() {
            this.inLoading = true;
            this.isLoaded = false;
            try {
                if (this.searchInContent) {
                    // Recherche dans le contenu
                    this.result = await invoke('search_content', {
                        query: this.search,
                        limit: this.contentSearchLimit
                    });
                } else {
                    // Recherche dans les métadonnées (comportement existant)
                    this.result = await invoke('search_files', {
                        search: this.search,
                        types: this.types,
                        isDir: this.isDir,
                        folders: this.folders,
                        sizeLimit: this.sizeLimit,
                        dateRange: this.dateRange,
                        dateMode: this.dateMode
                    });
                }
                this.isLoaded = true;
            } catch (error) {
                console.error(error);
            } finally {
                this.inLoading = false;
            }
        },

        async getIndexStats() {
            try {
                const stats = await invoke('get_index_stats');
                return stats;
            } catch (error) {
                console.error('Erreur récupération stats indexation:', error);
                return null;
            }
        },

        async startIndexation() {
            try {
                await invoke('index_files');
            } catch (error) {
                console.error('Erreur démarrage indexation:', error);
            }
        },

        reset_search() {
            this.search = '';
            this.searchResult = '';
            this.types = [];
            this.folders = [];
            this.sizeLimit = [0, 1000];
            this.dateRange = [0, 9999999999];
            this.dateMode = 'modify';
            this.isDir = false;
            this.inLoading = false;
            this.result = [];
            this.isLoaded = false;
            this.showPath = true;
            this.searchInPath = false;
            this.autoSubmit = true;
        },

        async openFile(path: string) {
            await invoke('open_file_in_explorer', { path: path });
        },

        async copyPath(path: string) {
            try {
                await navigator.clipboard.writeText(path);
            } catch (error) {
                console.error('Erreur lors de la copie dans le presse-papiers:', error);
            }
        },

        async exportToCsv() {
            if (this.result.length === 0) {
                console.warn('Aucun résultat à exporter');
                return;
            }

            // En-têtes CSV en français
            const headers = ['Chemin', 'Nom', 'Type', 'Taille (octets)', 'Dernière modification', 'Date de création', 'Est un dossier'];
            
            // Fonction pour échapper les valeurs CSV (gérer les virgules et guillemets)
            const escapeCsvValue = (value: any): string => {
                if (value === null || value === undefined) return '';
                const stringValue = String(value);
                if (stringValue.includes(',') || stringValue.includes('"') || stringValue.includes('\n')) {
                    return `"${stringValue.replace(/"/g, '""')}"`;
                }
                return stringValue;
            };

            const csvRows = [
                headers.join(','),
                ...this.result.map(file => [
                    escapeCsvValue(file.path),
                    escapeCsvValue(file.name),
                    escapeCsvValue(file.file_type || ''),
                    escapeCsvValue(file.size || 0),
                    escapeCsvValue(file.last_modified || ''),
                    escapeCsvValue(file.created_at || ''),
                    escapeCsvValue(file.is_dir ? 'Oui' : 'Non')
                ].join(','))
            ];

            const csvContent = csvRows.join('\n');
            
            // Ajouter BOM pour l'encodage UTF-8 (important pour Excel)
            const BOM = '\uFEFF';
            const blob = new Blob([BOM + csvContent], { type: 'text/csv;charset=utf-8;' });
            
            // Créer le lien de téléchargement
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = `recherche_${new Date().toISOString().split('T')[0]}.csv`;
            a.style.display = 'none';
            
            // Ajouter au DOM, cliquer et nettoyer
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
            URL.revokeObjectURL(url);

            console.log(`CSV exporté avec succès: ${this.result.length} fichiers`);
        }
    }
});