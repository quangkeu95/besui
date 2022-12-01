pub mod exchange;
pub mod token;
use async_trait::async_trait;

pub struct ServiceId(String);

#[async_trait]
pub trait Service: Send + Sync {
    async fn start(&mut self) -> anyhow::Result<()>;
}

pub struct ServiceControl {}

impl ServiceControl {
    pub fn new() -> Self {
        ServiceControl {}
    }
}
