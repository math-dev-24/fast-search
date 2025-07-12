use std::{path::PathBuf, time::SystemTime};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub file_type: Option<String>,
    pub size: Option<u64>,
    pub last_modified: SystemTime,
    pub created_at: SystemTime,
    pub is_indexed: bool,
    pub content_indexed: bool,
    pub is_indexable: bool,
    pub content_hash: Option<String>,
}