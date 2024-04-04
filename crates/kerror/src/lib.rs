use std::io;

#[derive(Debug, thiserror::Error)]
pub enum KError {
    #[error(transparent)]
    IO(#[from] io::Error),

    #[error(transparent)]
    TomlDeserialize(#[from] toml::de::Error),

    #[error("{0}")]
    KError(String),
}

pub type KResult<T, E = KError> = Result<T, E>;
