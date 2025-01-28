use faststr::FastStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("command name illegal: {hint}")]
    CommandNameIllegal { hint: FastStr },
    #[error("param parse failed: {hint}")]
    Param { hint: FastStr },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

pub(super) type Result<T> = std::result::Result<T, Error>;
