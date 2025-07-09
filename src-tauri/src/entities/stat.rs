use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Stat {
    pub nb_folders: u32,
    pub nb_files: u32,
    pub total_size: u64
}