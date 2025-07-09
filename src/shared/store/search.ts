import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { useMessage } from 'naive-ui';
import type { File } from '../../types/file';


type SearchState = {
    search: string;
    searchResult: string;
    types: string[];
    folders: string[];
    isDir: boolean;
    inLoading: boolean;
    result: File[];
    isLoaded: boolean;
    showPath: boolean;
    searchInPath: boolean;
}

export const useSearchStore = defineStore('search', {
    state: (): SearchState => ({
        search: '',
        searchResult: '',
        types: [],
        folders: [],
        isDir: false,
        inLoading: false,
        result: [],
        isLoaded: false,
        showPath: true,
        searchInPath: false
    }),

    getters: {
        filterResult: (state): File[] => {
            return state.result.filter(
                file => file.name.toLowerCase().includes(state.searchResult.toLowerCase())
            );
        },

        options: (state): { label: string; value: string }[] => {
            const uniqueNames = [...new Set(state.result.map(file => file.name))];
            return uniqueNames.slice(0, 5).map(name => ({
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
                    folders: this.folders
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
        },

        async openFile(path: string) {
            await invoke('open_file_in_explorer', { path: path });
        }
    }
});