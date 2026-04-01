export type AiProvider = 'lm_studio' | 'ollama' | 'open_ai' | 'anthropic' | 'mistral';

export interface CloudCredentialsRef {
  credential_id: string;
}

export interface AiProviderConfig {
  provider: AiProvider;
  endpoint: string;
  model?: string | null;
  credential_ref?: CloudCredentialsRef | null;
}
