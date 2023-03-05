#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Web(#[from] reqwest::Error),

    #[error(transparent)]
    Orimpl(#[from] OrimplError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum OrimplError {
    #[error("无法找到图片大小")]
    ImgLenNotFound,
}
