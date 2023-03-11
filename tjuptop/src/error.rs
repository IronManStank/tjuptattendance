//! asd

/// asd
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// IO 错误
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// 其他任何错误
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
