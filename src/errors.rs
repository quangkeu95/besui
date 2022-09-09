use config::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
