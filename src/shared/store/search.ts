import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import type { File, Stat } from '../../types';
import { type SearchQuery, SortBy, SortOrder, DateMode } from '../../types/search';
import { DateTime } from 'luxon';

type SearchState = {
    query: SearchQuery;
    inLoading: boolean;
    result: File[];
    isLoaded: boolean;
    search: string;
    auto_submit: boolean;
}

export const useSearchStore = defineStore('search', {
    state: (): SearchState => ({
                    query: {
                text: '',
                filters: {
                    is_dir: false,
                    folders: [],
                    file_types: [],
                    size_limit: [0, 1000],
                    date_range: [0, DateTime.now().endOf('day').toMillis()],
                    date_mode: DateMode.MODIFY,
                    search_in_content: false
                },
                sort_by: SortBy.NAME,
                sort_order: SortOrder.ASC,
                limit: 1000,
                offset: 0,
                search_in_content: false,
                path_pattern: null
            },
        inLoading: false,
        isLoaded: false,
        result: [],
        search: '',
        auto_submit: true
    }),

    getters: {
        filterResult(): File[] {
            return this.result.filter(
                file => file.name.toLowerCase().includes(this.search.toLowerCase())
            );
        },

        options(): { label: string; value: string }[] {
            const allNames = this.result.map(file => file.name);
            const uniqueNames = [...new Set(allNames)];
            
            const filteredSuggestions = uniqueNames.filter(name => 
                name.toLowerCase().includes(this.search.toLowerCase())
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
                console.log(JSON.stringify(this.query));
                this.result = await invoke('search_files', { query: this.query });
                this.isLoaded = true;
            } catch (error) {
                console.error(error);
            } finally {
                this.inLoading = false;
            }
        },

        async getIndexStats() {
            try {
                const stats = await invoke<Stat>('get_index_stats');
                console.log(stats);
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
            this.query = {
                text: '',
                filters: {
                    is_dir: false,
                    folders: [],
                    file_types: [],
                    size_limit: [0, 1000],
                    date_range: [0, DateTime.now().endOf('day').toMillis()],
                    date_mode: DateMode.MODIFY,
                    search_in_content: false
                },
                sort_by: SortBy.NAME,
                sort_order: SortOrder.ASC,
                limit: 100,
                offset: 0,
                search_in_content: false,
                path_pattern: null
            };
            this.result = [];
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

            const headers = ['Chemin', 'Nom', 'Type', 'Taille (octets)', 'Dernière modification', 'Date de création', 'Est un dossier'];
            
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
            
            const BOM = '\uFEFF';
            const blob = new Blob([BOM + csvContent], { type: 'text/csv;charset=utf-8;' });
            
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = `recherche_${new Date().toISOString().split('T')[0]}.csv`;
            a.style.display = 'none';
            
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
            URL.revokeObjectURL(url);

            console.log(`CSV exporté avec succès: ${this.result.length} fichiers`);
        }
    }
});