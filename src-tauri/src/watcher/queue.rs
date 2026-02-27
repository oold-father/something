use super::event::FileEvent;
use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

/// 事件队列配置
#[derive(Debug, Clone)]
pub struct QueueConfig {
    /// 队列最大容量
    pub max_capacity: usize,
    /// 防抖延迟时间（毫秒）
    pub debounce_delay_ms: u64,
    /// 批处理大小
    pub batch_size: usize,
}

impl Default for QueueConfig {
    fn default() -> Self {
        QueueConfig {
            max_capacity: 10000,
            debounce_delay_ms: 500,
            batch_size: 100,
        }
    }
}

/// 事件队列
pub struct EventQueue {
    sender: mpsc::Sender<FileEvent>,
    receiver: mpsc::Receiver<FileEvent>,
    config: QueueConfig,
    processed_paths: HashSet<String>,
}

impl EventQueue {
    /// 创建新的事件队列
    pub fn new(config: QueueConfig) -> Self {
        let (sender, receiver) = mpsc::channel(config.max_capacity);

        EventQueue {
            sender,
            receiver,
            config,
            processed_paths: HashSet::new(),
        }
    }

    /// 发送事件
    pub async fn send(&self, event: FileEvent) -> Result<(), FileEvent> {
        self.sender.send(event).await.map_err(|e| e.0)
    }

    /// 尝试立即发送事件（不等待）
    pub fn try_send(&self, event: FileEvent) -> Result<(), FileEvent> {
        self.sender.try_send(event).map_err(|e| e.0)
    }

    /// 接收事件
    pub async fn recv(&mut self) -> Option<FileEvent> {
        self.receiver.recv().await
    }

    /// 接收一批事件（用于批处理）
    pub async fn recv_batch(&mut self) -> Vec<FileEvent> {
        let mut batch = Vec::with_capacity(self.config.batch_size);

        // 等待第一个事件
        if let Some(event) = self.recv().await {
            batch.push(event);

            // 收集后续事件，直到队列为空或达到批处理大小
            while batch.len() < self.config.batch_size {
                match self.receiver.try_recv() {
                    Ok(event) => batch.push(event),
                    Err(_) => break,
                }
            }
        }

        batch
    }

    /// 接收并去重事件（防抖处理）
    pub async fn recv_deduplicated(&mut self) -> Vec<FileEvent> {
        let mut batch = self.recv_batch().await;
        let mut deduped = Vec::new();
        let mut path_map: std::collections::HashMap<String, FileEvent> = std::collections::HashMap::new();

        // 去重：同一路径只保留最后的事件
        for event in batch {
            if let Some(path) = event.primary_path() {
                let path_str = path.to_string_lossy().to_string();
                path_map.insert(path_str, event);
            } else {
                deduped.push(event);
            }
        }

        // 合并去重后的事件和未被处理的事件
        for event in path_map.into_values() {
            deduped.push(event);
        }

        deduped
    }

    /// 获取队列发送器
    pub fn sender(&self) -> mpsc::Sender<FileEvent> {
        self.sender.clone()
    }

    /// 检查队列是否为空
    pub fn is_empty(&self) -> bool {
        self.receiver.is_empty()
    }

    /// 获取队列长度
    pub fn len(&self) -> usize {
        self.receiver.len()
    }

    /// 关闭队列
    pub fn close(&self) {
        self.sender.close();
    }
}

/// 事件处理器 trait
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    /// 处理单个事件
    async fn handle(&self, event: FileEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 批量处理事件
    async fn handle_batch(&self, events: Vec<FileEvent>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        for event in events {
            self.handle(event).await?;
        }
        Ok(())
    }
}

/// 防抖包装器
pub struct DebouncedHandler<H> {
    inner: H,
    delay: Duration,
    last_events: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, FileEvent>>>,
}

impl<H: EventHandler + Clone> DebouncedHandler<H> {
    pub fn new(handler: H, delay_ms: u64) -> Self {
        DebouncedHandler {
            inner: handler,
            delay: Duration::from_millis(delay_ms),
            last_events: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl<H: EventHandler + Send + Sync + Clone> EventHandler for DebouncedHandler<H> {
    async fn handle(&self, event: FileEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(path) = event.primary_path() {
            let path_str = path.to_string_lossy().to_string();
            let delay = self.delay;
            let inner = self.inner.clone();
            let last_events = self.last_events.clone();

            // 存储事件
            {
                let mut events = last_events.lock().unwrap();
                events.insert(path_str.clone(), event.clone());
            }

            // 延迟后处理
            tokio::spawn(async move {
                sleep(delay).await;

                let event_to_handle = {
                    let mut events = last_events.lock().unwrap();
                    events.remove(&path_str)
                };

                if let Some(evt) = event_to_handle {
                    let _ = inner.handle(evt).await;
                }
            });

            Ok(())
        } else {
            self.inner.handle(event).await
        }
    }
}
