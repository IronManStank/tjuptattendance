#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
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
