use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error initializing RootResolver, instance is already initialized")]
    ErrorInitializingRootResolver,
    #[error("error RootResolver is not initialized")]
    ErrorRootResolverNotInitialized,
    #[error("token id cannot be empty")]
    TokenIdCannotBeEmpty,
    #[error("token symbol cannot be empty")]
    TokenSymbolCannotBeEmpty,
    #[error("token name cannot be empty")]
    TokenNameCannotBeEmpty,
    #[error("{0}")]
    DatabaseError(DatabaseErrors),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum TokenErrors {
    #[error("list token cannot be empty")]
    ListTokenEmpty,
}

#[derive(Error, Debug)]
pub enum DatabaseErrors {
    #[error("cannot convert database connection")]
    CannotConvertDatabaseConnection,
}
