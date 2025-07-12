export interface File {
    path: string;
    name: string;
    is_dir: boolean;
    file_type: string | null;
    size: number | null;
    last_modified: string | null;
    created_at: string | null;
    is_indexed: boolean;
    content_indexed: boolean;
    is_indexable: boolean;
    content_hash: string | null;
}