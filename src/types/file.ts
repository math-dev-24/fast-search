export interface File {
    path: string;
    name: string;
    is_dir: boolean;
    file_type: string | null;
    size: number | null;
    last_modified: string | null;
    created_at: string | null;
}