use tracing::info;

use crate::resolver::RootResolver;

use self::exchange_monitor::ExchangeMonitor;
use std::sync::Arc;

pub mod exchange_monitor;

pub struct MonitorResolver {
    exchange_monitor: Arc<ExchangeMonitor>,
}

impl MonitorResolver {
    pub fn new() -> Self {
        MonitorResolver {
            exchange_monitor: Arc::new(ExchangeMonitor::new()),
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        info!("Starting monitor resolver...");
        self.exchange_monitor.start().await
    }
}
