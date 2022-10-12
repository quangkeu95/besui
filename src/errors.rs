use config::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
    #[error("error chain ID {0} not supported")]
    ChainNotSupported(u64),
    #[error("no default provider for chain ID {0}")]
    NoDefaultProvider(u64),
    #[error("provider not found")]
    ProviderNotFound,
    #[error("receiver dropped error")]
    ReceiverDroppedError,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
