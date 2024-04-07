mod rkyv;

#[derive(Debug, thiserror::Error)]
pub enum KError {
    #[error(transparent)]
    StdIO(#[from] std::io::Error),

    #[error(transparent)]
    StdEnvVar(#[from] std::env::VarError),

    #[error(transparent)]
    TomlDeserialize(#[from] toml::de::Error),

    #[error(transparent)]
    RkyvCompositeSerializer(#[from] rkyv::CompositeSerializer),

    #[error("{0}")]
    KError(String),
}

pub type KResult<T, E = KError> = Result<T, E>;
