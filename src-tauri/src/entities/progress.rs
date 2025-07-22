use std::time::{Duration, Instant};

const PROGRESS_UPDATE_INTERVAL: Duration = Duration::from_millis(500);
const PATH_PROCESSING_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes par chemin

#[derive(Debug, Clone)]
pub struct ScanProgressTracker {
    pub total_paths: usize,
    pub current_path_index: usize,
    pub total_files: usize,
    pub last_progress_update: Instant,
    pub start_time: Instant,
}


impl ScanProgressTracker {
    pub fn new(total_paths: usize) -> Self {
        Self {
            total_paths,
            current_path_index: 0,
            total_files: 0,
            last_progress_update: Instant::now(),
            start_time: Instant::now(),
        }
    }

    pub fn update_path_progress(&mut self, path_progress: f64) -> f64 {
        let path_weight = 1.0 / self.total_paths as f64;
        let current_path_contribution = (self.current_path_index as f64 + path_progress) * path_weight;
        current_path_contribution
    }

    pub fn next_path(&mut self) {
        self.current_path_index += 1;
    }

    pub fn set_total_files(&mut self, total: usize) {
        self.total_files = total;
    }

    pub fn should_update_progress(&self) -> bool {
        self.last_progress_update.elapsed() >= PROGRESS_UPDATE_INTERVAL
    }

    pub fn update_progress_time(&mut self) {
        self.last_progress_update = Instant::now();
    }

    pub fn is_timeout(&self) -> bool {
        self.start_time.elapsed() > PATH_PROCESSING_TIMEOUT
    }
}