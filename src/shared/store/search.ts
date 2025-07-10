import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { useMessage } from 'naive-ui';
import type { File } from '../../types/file';


type SearchState = {
    search: string;
    searchResult: string;
    types: string[];
    folders: string[];
    sizeLimit: [number, number];
    dateLimit: [number, number];
    isDir: boolean;
    inLoading: boolean;
    result: File[];
    isLoaded: boolean;
    showPath: boolean;
    searchInPath: boolean;
    autoSubmit: boolean;
}

export const useSearchStore = defineStore('search', {
    state: (): SearchState => ({
        search: '',
        searchResult: '',
        types: [],
        folders: [],
        sizeLimit: [0, 10],
        dateLimit: [new Date(0).getTime(), new Date().getTime()],
        isDir: false,
        inLoading: false,
        isLoaded: false,
        showPath: true,
        searchInPath: false,
        result: [],
        autoSubmit: true
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
                this.result = await invoke('search_files', {
                    search: this.search,
                    types: this.types,
                    isDir: this.isDir,
                    folders: this.folders,
                    sizeLimit: this.sizeLimit.map(size => (size * 1024 * 1024).toString()).join(','),
                    dateLimit: this.dateLimit.map(date => date.toString()).join(',')
                });
                this.isLoaded = true;
                this.send_message(this.result.length + ' résultats trouvés.', 'success');
            } catch (error) {
                console.error(error);
            } finally {
                this.inLoading = false;
            }
        },

        send_message(tmp_message: string, tmp_type: 'success' | 'error' | 'warning' | 'info') {
            const message = useMessage();
            message[tmp_type](tmp_message);
        },

        reset_search() {
            this.search = '';
            this.searchResult = '';
            this.types = [];
            this.folders = [];
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
                this.send_message('Chemin copié dans le presse-papiers.', 'success');
            } catch (error) {
                console.error('Erreur lors de la copie dans le presse-papiers:', error);
            }
        }
    }
});