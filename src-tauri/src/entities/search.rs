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

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            text: String::new(),
            filters: SearchFilters::default(),
            sort_by: SortBy::Name,
            sort_order: SortOrder::Asc,
            limit: 10,
            offset: 0,
            search_in_content: false,
            path_pattern: None,
        }
    }
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

impl Default for SearchFilters {
    fn default() -> Self {
        Self {
            is_dir: false,
            folders: Vec::new(),
            file_types: Vec::new(),
            size_limit: [0, 0],
            date_range: [0, 0],
            date_mode: DateMode::Create,
            search_in_content: false,
        }
    }
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