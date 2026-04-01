import type { AiProvider } from "./ai";

export interface Setting {
    search_path: string[];
    ai_provider: AiProvider;
    ai_endpoint: string;
    ai_default_model: string;
}