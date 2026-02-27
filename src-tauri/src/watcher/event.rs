use std::path::PathBuf;

/// 文件系统事件类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileEvent {
    /// 文件创建
    Created { path: PathBuf },
    /// 文件修改
    Modified { path: PathBuf },
    /// 文件删除
    Deleted { path: PathBuf },
    /// 文件移动/重命名
    Moved { from: PathBuf, to: PathBuf },
    /// 递归扫描开始
    ScanStart { path: PathBuf },
    /// 递归扫描结束
    ScanEnd { path: PathBuf, count: usize },
    /// 错误事件
    Error { path: PathBuf, error: String },
}

impl FileEvent {
    /// 获取事件的主要路径
    pub fn primary_path(&self) -> Option<&PathBuf> {
        match self {
            FileEvent::Created { path } => Some(path),
            FileEvent::Modified { path } => Some(path),
            FileEvent::Deleted { path } => Some(path),
            FileEvent::Moved { from, .. } => Some(from),
            FileEvent::ScanStart { path } => Some(path),
            FileEvent::ScanEnd { path, .. } => Some(path),
            FileEvent::Error { path, .. } => Some(path),
        }
    }

    /// 判断是否是扫描事件
    pub fn is_scan_event(&self) -> bool {
        matches!(self, FileEvent::ScanStart { .. } | FileEvent::ScanEnd { .. })
    }

    /// 判断是否是错误事件
    pub fn is_error(&self) -> bool {
        matches!(self, FileEvent::Error { .. })
    }
}
