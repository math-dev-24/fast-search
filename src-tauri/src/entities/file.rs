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
    pub accessed_at: SystemTime,
    pub is_indexed: bool,
    pub content_indexed: bool,
    pub is_indexable: bool,
    pub is_hidden: bool,
    pub is_readonly: bool,
    pub is_system: bool,
    pub is_executable: bool,
    pub is_symlink: bool,
    pub permissions: Option<u32>,
    pub owner: Option<String>,
    pub group: Option<String>,
    pub mime_type: Option<String>,
    pub encoding: Option<String>,
    pub line_count: Option<u32>,
    pub word_count: Option<u32>,
    pub checksum: Option<String>,
    pub is_encrypted: bool,
}