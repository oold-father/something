mod event;
mod queue;
mod scanner;

#[cfg(test)]
mod tests;

pub use event::FileEvent;
pub use queue::{EventQueue, EventHandler, DebouncedHandler, QueueConfig};
pub use scanner::{DirectoryScanner, ScanResult, ScanConfig};

use crate::db::Database;
use crate::error::Result;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

/// 文件监控器
pub struct FileWatcher {
    watcher: RecommendedWatcher,
    event_sender: mpsc::Sender<FileEvent>,
    watched_paths: HashSet<PathBuf>,
    is_running: Arc<std::sync::atomic::AtomicBool>,
}

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

/// 文件监控管理器
pub struct WatcherManager {
    watcher: FileWatcher,
    event_queue: EventQueue,
    task_handle: Option<JoinHandle<()>>,
}

impl WatcherManager {
    /// 创建新的监控管理器
    pub fn new() -> Result<Self> {
        let watcher = FileWatcher::new()?;
        let event_queue = EventQueue::new(QueueConfig::default());

        Ok(WatcherManager {
            watcher,
            event_queue,
            task_handle: None,
        })
    }

    /// 启动监控
    pub async fn start<H: EventHandler + Send + Sync + 'static>(
        &mut self,
        handler: H,
    ) -> Result<()> {
        if self.task_handle.is_some() {
            return Ok(()); // 已在运行
        }

        let mut rx = self.event_queue.sender.clone();

        let handle = tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                let _ = handler.handle(event).await;
            }
        });

        self.task_handle = Some(handle);

        Ok(())
    }

    /// 停止监控
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(handle) = self.task_handle.take() {
            handle.abort();
        }
        self.event_queue.close();
        Ok(())
    }

    /// 添加监控路径
    pub async fn add_watch(&mut self, path: &Path, recursive: bool) -> Result<()> {
        self.watcher.watch(path, recursive)?;

        // 发送扫描开始事件
        let _ = self.event_queue.send(FileEvent::ScanStart {
            path: path.to_path_buf(),
        }).await;

        Ok(())
    }

    /// 移除监控路径
    pub async fn remove_watch(&mut self, path: &Path) -> Result<()> {
        self.watcher.unwatch(path)?;
        Ok(())
    }

    /// 获取事件队列
    pub fn event_queue(&self) -> &EventQueue {
        &self.event_queue
    }
}

impl Drop for WatcherManager {
    fn drop(&mut self) {
        // 停止所有监控
        self.watcher.unwatch_all();
        self.event_queue.close();
    }
}

#[async_trait::async_trait]
impl EventHandler for Database {
    async fn handle(&self, event: FileEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::db::{File, FileStatus, FileType};

        match event {
            FileEvent::Created { path } | FileEvent::Modified { path } => {
                // 处理文件创建/修改
                if let Ok(metadata) = std::fs::metadata(&path) {
                    if metadata.is_file() {
                        let path_str = path.to_string_lossy().to_string();
                        let name = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string();

                        let extension = path.extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("")
                            .to_string();

                        let file_type = FileType::from_extension(&extension);

                        // 创建或更新文件记录
                        let existing_file = self.get_file_by_path(&path_str)?;

                        if existing_file.is_none() {
                            let file = File {
                                id: None,
                                path: path_str,
                                name,
                                extension,
                                size: metadata.len(),
                                file_type,
                                created_at: chrono::Utc::now(),
                                modified_at: chrono::Utc::now(),
                                accessed_at: chrono::Utc::now(),
                                status: FileStatus::Active,
                                indexed_at: chrono::Utc::now(),
                                metadata: None,
                            };

                            let file_id = self.create_file(&file)?;

                            // 添加自动标签
                            let _ = self.add_tag_to_file_by_name(file_id, file_type.display_name(), true);
                        }
                    }
                }
            }
            FileEvent::Deleted { path } => {
                // 处理文件删除
                let path_str = path.to_string_lossy().to_string();
                if let Some(file) = self.get_file_by_path(&path_str)? {
                    if let Some(id) = file.id {
                        self.update_file_status(id, FileStatus::Deleted)?;
                    }
                }
            }
            FileEvent::Moved { from, to } => {
                // 处理文件移动
                let from_str = from.to_string_lossy().to_string();
                let to_str = to.to_string_lossy().to_string();

                if let Some(mut file) = self.get_file_by_path(&from_str)? {
                    file.path = to_str.clone();
                    file.name = to.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    file.extension = to.extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("")
                        .to_string();

                    // 更新文件记录（简化处理，实际应该有专门的 update 方法）
                    let _ = self.delete_file(file.id.unwrap_or(0));
                    let _ = self.create_file(&file);
                }
            }
            _ => {}
        }

        Ok(())
    }
}
