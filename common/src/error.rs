#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Web(#[from] reqwest::Error),

    #[error(transparent)]
    Data(#[from] DouBanDataError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// 与豆瓣数据获取/转换的相关错误
#[derive(Debug, thiserror::Error)]
pub enum DouBanDataError {
    #[error("无法找到图片大小")]
    ImgLenNotFound,
    #[error("过期的海报")]
    OutDated,
    #[error("无法找到海报")]
    PosterNotFound,
    #[error("API获取信息失败")]
    ApiTired,
}
