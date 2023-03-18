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

    /// DoubanData 相关错误
    #[error(transparent)]
    Data(#[from] DouBanDataError),

    /// 其他任何错误
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Answer 相关错误
#[derive(Debug, thiserror::Error)]
pub enum DouBanDataError {
    /// 无法找到图片大小
    #[error("无法找到图片大小，可能是 API 服务器未设置 Content-Length")]
    ImgLenNotFund,

    /// 图片格式错误
    #[error("仅支持 Jpeg 格式")]
    ImgFormatError,

    /// 无法快速检查答案
    #[error("无足够信息来快速匹配")]
    InfoNotEnoughError,
}
