export interface Stat {
    nb_folders: number;
    nb_files: number;
    total_size: number;
    indexed_files: number;
    unindexed_files: number;
    indexed_percentage: number;
    content_indexed_files: number;
    uncontent_indexed_files: number;
    content_indexed_percentage: number;
    unindexable_files: number;
}