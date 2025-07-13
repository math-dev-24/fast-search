use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub text: String,
    pub filters: SearchFilters,
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
    pub limit: u32,
    pub offset: u32,
    pub search_in_content: bool,
    pub path_pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    pub is_dir: bool,
    pub folders: Vec<String>,
    pub file_types: Vec<String>,
    pub size_limit: [u32; 2],
    pub date_range: [u64; 2],
    pub date_mode: DateMode,
    pub search_in_content: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortBy {
    Name,
    Size,
    LastModified,
    CreatedAt,
    AccessedAt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DateMode {
    Create,
    Modify,
}