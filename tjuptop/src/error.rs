//! 本 crate 所有错误

/// 本 crate 所有错误
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// IO 错误
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// 网络错误
    #[error(transparent)]
    Web(#[from] reqwest::Error),

    /// util error
    #[error(transparent)]
    Answer(#[from] util::error::AnswerError),

    /// Api Error
    #[error(transparent)]
    Api(#[from] ApiError),

    /// cookies Error
    #[error("cookies error")]
    CookiesError,

    /// 其他任何错误
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// API 相关错误
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// 无法解析API数据
    #[error("无法解析 API 返回的数据")]
    DataParser,

    /// API 无有效返回 或 状态码错误
    #[error("API 无响应或无有效载荷")]
    Tired,
}
