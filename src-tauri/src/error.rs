/// 应用错误类型
#[derive(Debug)]
pub enum AppError {
    Database(rusqlite::Error),
    Io(std::io::Error),
    Notify(notify::Error),
    #[allow(dead_code)]
    FileNotFound(String),
    #[allow(dead_code)]
    TagNotFound(String),
    #[allow(dead_code)]
    AlreadyWatched(String),
    #[allow(dead_code)]
    PermissionDenied(String),
    ConfigNotFound(String),
    #[allow(dead_code)]
    Unknown(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Database(e) => write!(f, "数据库错误: {}", e),
            AppError::Io(e) => write!(f, "IO 错误: {}", e),
            AppError::Notify(e) => write!(f, "文件监控错误: {}", e),
            AppError::FileNotFound(s) => write!(f, "文件不存在: {}", s),
            AppError::TagNotFound(s) => write!(f, "标签不存在: {}", s),
            AppError::AlreadyWatched(s) => write!(f, "目录已监控: {}", s),
            AppError::PermissionDenied(s) => write!(f, "权限不足: {}", s),
            AppError::ConfigNotFound(s) => write!(f, "未找到配置: {}", s),
            AppError::Unknown(s) => write!(f, "未知错误: {}", s),
        }
    }
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, AppError>;

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Database(e) => Some(e),
            AppError::Io(e) => Some(e),
            AppError::Notify(e) => Some(e),
            _ => None,
        }
    }
}

// 手动实现 From 转换
impl From<rusqlite::Error> for AppError {
    fn from(error: rusqlite::Error) -> Self {
        AppError::Database(error)
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<notify::Error> for AppError {
    fn from(error: notify::Error) -> Self {
        AppError::Notify(error)
    }
}
