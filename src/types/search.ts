export interface SearchFilters {
    is_dir: boolean;
    folders: string[];
    file_types: string[];
    size_limit: [number, number];
    date_range: [number, number];
    date_mode: DateMode;
    search_in_content: boolean;
}

export enum SortBy {
    NAME = 'Name',
    SIZE = 'Size',
    LAST_MODIFIED = 'LastModified',
    CREATED_AT = 'CreatedAt',
    ACCESSED_AT = 'AccessedAt',
}

export enum SortOrder {
    ASC = 'Asc',
    DESC = 'Desc',
}

export enum DateMode {
    CREATE = 'Create',
    MODIFY = 'Modify',
}

export interface SearchQuery {
    text: string;
    filters: SearchFilters;
    sort_by: SortBy;
    sort_order: SortOrder;
    limit: number;
    offset: number;
    search_in_content: boolean;
    path_pattern: string | null;
}