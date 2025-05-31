use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use fyrox::core::watcher::FileSystemWatcher;

pub enum LazyWatcher {
    Initialized(FileSystemWatcher),
    TryingToInitialize {
        path: PathBuf,
        duration: Duration,
        last_checked_at: SystemTime
    },
}