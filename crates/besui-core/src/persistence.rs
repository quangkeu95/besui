use async_trait::async_trait;
use downcast_rs::{impl_downcast, DowncastSync};
use mockall::automock;
use std::sync::Arc;

pub mod postgres;

/**
 * An interface for persistence storage
 */
#[async_trait]
#[automock]
pub trait Persistence: Send + Sync {
    async fn new_connection_pool(&self) -> anyhow::Result<SharedConnection>;
}

pub type SharedPersistence = Arc<dyn Persistence>;

/**
 * An connection pool can be used
 */
pub trait ConnectionPool: Send + Sync + DowncastSync {}
impl_downcast!(sync ConnectionPool);

pub type SharedConnection = Arc<dyn ConnectionPool>;

pub trait Transaction {}
