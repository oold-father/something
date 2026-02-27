use thiserror::Error;

/// 应用错误类型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("文件不存在: {0}")]
    FileNotFound(String),

    #[error("标签不存在: {0}")]
    TagNotFound(String),

    #[error("目录已监控: {0}")]
    AlreadyWatched(String),

    #[error("权限不足: {0}")]
    PermissionDenied(String),

    #[error("未找到配置: {0}")]
    ConfigNotFound(String),

    #[error("未知错误: {0}")]
    Unknown(String),
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, AppError>;

/// 将错误转换为字符串，用于 Tauri 命令
impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}
