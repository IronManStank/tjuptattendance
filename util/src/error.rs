//! 本 crate 中的所有错误及大体分类

/// 本 crate 中的所有错误
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // /// IO 错误
    // #[error(transparent)]
    // Io(#[from] std::io::Error),

    /// 网络错误
    #[error(transparent)]
    Web(#[from] reqwest::Error),

    /// Answer 相关错误
    #[error(transparent)]
    Answer(#[from] AnswerError),

    /// 其他任何错误
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Answer 相关错误
#[derive(Debug, thiserror::Error)]
pub enum AnswerError {
    /// 无法找到图片大小
    #[error("无法找到图片大小，可能是 API 服务器未设置 Content-Length")]
    ImgLenNotFund,
}
