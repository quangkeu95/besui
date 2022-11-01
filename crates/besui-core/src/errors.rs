use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("token id cannot be empty")]
    TokenIdCannotBeEmpty,
    #[error("token symbol cannot be empty")]
    TokenSymbolCannotBeEmpty,
    #[error("token name cannot be empty")]
    TokenNameCannotBeEmpty,
    #[error("token errors")]
    TokenErrors,
}

#[derive(Error, Debug)]
pub enum TokenErrors {
    #[error("list token cannot be empty")]
    ListTokenEmpty,
}
