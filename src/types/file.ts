export interface File {
    path: string;
    name: string;
    is_dir: boolean;
    file_type: string | null;
    size: number | null;
    last_modified: string | null;
    created_at: string | null;
    accessed_at: string | null;
    is_indexed: boolean;
    content_indexed: boolean;
    is_indexable: boolean;
    is_hidden: boolean;
    is_readonly: boolean;
    is_system: boolean;
    is_executable: boolean;
    is_symlink: boolean;
    permissions: number | null;
    owner: string | null;
    group: string | null;
    mime_type: string | null;
    encoding: string | null;
    line_count: number | null;
    word_count: number | null;
    checksum: string | null;
    is_encrypted: boolean;
}