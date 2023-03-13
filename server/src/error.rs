//! 本 crate 所有定义的错误

/// 本 crate 中的所有错误
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// 其他所有错误
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
