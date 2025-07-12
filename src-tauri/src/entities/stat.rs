use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Stat {
    pub nb_folders: u32,
    pub nb_files: u32,
    pub total_size: u64,
    pub indexed_files: u32,
    pub unindexed_files: u32,
    pub content_indexed_files: u32,
    pub uncontent_indexed_files: u32,
    pub unindexable_files: u32,
    pub indexed_percentage: f64,
    pub content_indexed_percentage: f64,
}