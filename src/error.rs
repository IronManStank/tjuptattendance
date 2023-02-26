#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Web(#[from] reqwest::Error),

    #[error("用户名或密码错误")]
    UserVerification,

    #[error(transparent)]
    Att(#[from] AttError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum AttError {
    #[error("已达最大重试次数：[{}]", .0.join(", "))]
    Tired(Vec<String>),
}
