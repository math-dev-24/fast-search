import {defineStore} from "pinia";
import {invoke} from "@tauri-apps/api/core";


interface SettingState {
    status: "Ok" | "Error" | 'Loading';
    inLoading: boolean;
    paths: string[];
    ai_path: string;
}


export const useSettingStore = defineStore("settingStore", {
    state: (): SettingState => ({
        status: "Ok",
        inLoading: false,
        paths: [],
        ai_path: "http://192.168.108.157:1234"
    }),

    actions: {
        async init() {
            await this.getAllPaths();
        },

        async getAllPaths() {
            try {
                this.inLoading = true;
                this.paths = await invoke<string[]>('get_all_paths');
                this.status = "Ok";
            } catch (e) {
                console.error(e);
                this.status = "Error";
            } finally {
                this.inLoading = false;
            }
        },

        async savePaths() {
            try {
                this.inLoading = true;
                await invoke('save_path', {paths: this.paths});
                this.status = "Ok";
            } catch (e) {
                console.error(e);
                this.status = "Error";
            } finally {
                this.inLoading = false;
            }
        },

        async resetSettings() {
            this.paths = [];
            this.ai_path = "http://192.168.108.157:1234";
            this.status = "Ok";
            await this.getAllPaths();
        }
    }
})