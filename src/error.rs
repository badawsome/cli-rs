use faststr::FastStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("command not found: {0}")]
    CommandNotFound(FastStr),
    #[error(transparent)]
    CommandBuildFail(#[from] super::command::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

pub(super) type Result<T> = std::result::Result<T, Error>;
