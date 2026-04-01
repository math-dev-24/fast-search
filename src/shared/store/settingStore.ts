import {defineStore} from "pinia";
import {invoke} from "@tauri-apps/api/core";
import type { AiProvider } from "../../types";


interface SettingState {
    status: "Ok" | "Error" | 'Loading';
    inLoading: boolean;
    paths: string[];
    ai_provider: AiProvider;
    ai_endpoint: string;
    ai_default_model: string;
}


export const useSettingStore = defineStore("settingStore", {
    state: (): SettingState => ({
        status: "Ok",
        inLoading: false,
        paths: [],
        ai_provider: "lm_studio",
        ai_endpoint: "http://localhost:1234",
        ai_default_model: ""
    }),

    actions: {
        async init() {
            await this.getAllPaths();
            this.loadAiSettings();
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
                await invoke('save_paths', {paths: this.paths});
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
            this.ai_provider = "lm_studio";
            this.ai_endpoint = "http://localhost:1234";
            this.ai_default_model = "";
            this.status = "Ok";
            this.saveAiSettings();
            await this.getAllPaths();
        },

        saveAiSettings() {
            localStorage.setItem(
                "ai_settings",
                JSON.stringify({
                    provider: this.ai_provider,
                    endpoint: this.ai_endpoint,
                    defaultModel: this.ai_default_model,
                }),
            );
        },

        loadAiSettings() {
            const raw = localStorage.getItem("ai_settings");
            if (!raw) return;

            try {
                const parsed = JSON.parse(raw) as {
                    provider?: AiProvider;
                    endpoint?: string;
                    defaultModel?: string;
                };

                this.ai_provider = parsed.provider ?? "lm_studio";
                this.ai_endpoint = parsed.endpoint ?? "http://localhost:1234";
                this.ai_default_model = parsed.defaultModel ?? "";
            } catch (error) {
                console.error("Failed to parse ai_settings from localStorage", error);
            }
        }
    }
})