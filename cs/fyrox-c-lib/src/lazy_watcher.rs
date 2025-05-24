use std::path::PathBuf;
use std::time::Duration;
use fyrox::core::watcher::FileSystemWatcher;

pub enum LazyWatcher {
    Initialized(FileSystemWatcher),
    TryingToInitialize {
        path: PathBuf,
        duration: Duration,
    },
}