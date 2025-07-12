use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ScanProgress {
    pub progress: f64,
    pub message: String,
    pub current_path: String,
}

#[derive(Serialize, Clone)]
pub struct ScanCollected {
    pub total: usize,
    pub message: String,
}

#[derive(Serialize, Clone)]
pub struct InsertProgress {
    pub progress: f64,
    pub processed: usize,
    pub total: usize,
}

#[derive(Serialize, Clone)]
pub struct ScanFinished {
    pub total: usize,
    pub message: String,
}

#[derive(Serialize, Clone)]
pub struct IndexProgress {
    pub progress: f64,
    pub message: String,
    pub processed: usize,
    pub total: usize,
}


#[derive(Serialize, Clone)]
pub struct IndexFinished {
    pub total: usize,
    pub message: String,
}