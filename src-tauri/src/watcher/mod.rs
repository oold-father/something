mod event;
mod queue;
mod scanner;

#[cfg(test)]
mod tests;

pub use scanner::{DirectoryScanner, ScanResult, ScanConfig, ScanError};

// FileWatcher 和相关事件是预留功能
#[allow(dead_code)]
pub use event::FileEvent;

use crate::error::Result;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc;

/// 文件监控器（预留功能）
#[allow(dead_code)]
pub struct FileWatcher {
    watcher: RecommendedWatcher,
    event_sender: mpsc::Sender<FileEvent>,
    watched_paths: HashSet<PathBuf>,
    is_running: Arc<std::sync::atomic::AtomicBool>,
}

/// 文件监控器实现（预留功能）
#[allow(dead_code)]
impl FileWatcher {
    /// 创建新的文件监控器
    pub fn new() -> Result<Self> {
        let (tx, _rx) = mpsc::channel(1000);
        let is_running = Arc::new(std::sync::atomic::AtomicBool::new(false));

        // 创建 notify watcher
        let watcher = notify::recommended_watcher({
            let tx = tx.clone();
            move |res: notify::Result<notify::Event>| {
                if let Ok(event) = res {
                    let file_events = Self::convert_notify_event(event);
                    for fe in file_events {
                        let _ = tx.try_send(fe);
                    }
                }
            }
        })?;

        Ok(FileWatcher {
            watcher,
            event_sender: tx,
            watched_paths: HashSet::new(),
            is_running,
        })
    }

    /// 监控指定路径
    pub fn watch(&mut self, path: &Path, recursive: bool) -> Result<()> {
        let mode = if recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        self.watcher.watch(path, mode)?;
        self.watched_paths.insert(path.to_path_buf());

        Ok(())
    }

    /// 停止监控指定路径
    pub fn unwatch(&mut self, path: &Path) -> Result<()> {
        self.watcher.unwatch(path)?;
        self.watched_paths.remove(path);

        Ok(())
    }

    /// 停止所有监控
    pub fn unwatch_all(&mut self) {
        for path in self.watched_paths.clone() {
            let _ = self.watcher.unwatch(&path);
        }
        self.watched_paths.clear();
    }

    /// 获取事件发送器
    pub fn event_sender(&self) -> mpsc::Sender<FileEvent> {
        self.event_sender.clone()
    }

    /// 获取监控的路径列表
    pub fn watched_paths(&self) -> &HashSet<PathBuf> {
        &self.watched_paths
    }

    /// 是否正在运行
    pub fn is_running(&self) -> bool {
        self.is_running.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// 转换 notify 事件为自定义事件
    fn convert_notify_event(event: notify::Event) -> Vec<FileEvent> {
        let mut file_events = Vec::new();

        for path in event.paths {
            let file_event = match event.kind {
                EventKind::Create(_) => FileEvent::Created { path },
                EventKind::Modify(_) => FileEvent::Modified { path },
                EventKind::Remove(_) => FileEvent::Deleted { path },
                _ => continue,
            };

            file_events.push(file_event);
        }

        file_events
    }
}
