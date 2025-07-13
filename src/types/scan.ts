export type Phase = "idle" | "collecting" | "inserting" | "finished" | "error";


export interface IndexProgress {
    progress: number;
    message: string;
    processed: number;
    total: number;
}

export interface IndexFinished {
    total: number;
    message: string;
}

export interface InsertProgress {
    progress: number;
    processed: number;
    total: number;
}

export interface InsertFinished {
    total: number;
    message: string;
}