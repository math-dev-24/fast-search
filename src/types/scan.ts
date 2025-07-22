export type Phase = "idle" | "collecting" | "inserting" | "finished" | "error";


export interface ScanProgressPayload {
    progress: number;
    message: string;
    current_path: string;
}

export interface ScanCollectedPayload {
    total: number;
    message: string;
}

export interface InsertProgressPayload {
    progress: number;
    processed: number;
    total: number;
}

export interface ScanFinishedPayload {
    total: number;
    message: string;
}

export interface IndexProgressPayload {
    progress: number;
    message: string;
    processed: number;
    total: number;
}

export interface IndexFinishedPayload {
    total: number;
    message: string;
}

export interface ProcessState {
    isActive: boolean;
    progress: number;
    message: string;
    currentPath: string;
    total: number;
    processed: number;
    phase: Phase;
    error: string;
    success: boolean;
}

export interface ScanDetails {
    name: string;
    icon: any;
    isActive: boolean;
    progress: number;
    message: string;
    currentPath: string;
    phase: string;
    error: string;
    success: boolean;
}