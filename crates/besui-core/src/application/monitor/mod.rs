use tracing::info;

use crate::resolver::RootResolver;

use self::exchange_monitor::ExchangeMonitor;
use std::sync::Arc;

pub mod exchange_monitor;

pub struct MonitorResolver {
    exchange_monitor: Arc<ExchangeMonitor>,
}

impl Default for MonitorResolver {
    fn default() -> Self {
        // let root_resolver = RootResolver::get().unwrap();
        // let db_conn = root_resolver.db_conn.clone();

        MonitorResolver {
            exchange_monitor: Arc::new(ExchangeMonitor::new()),
        }
    }
}

impl MonitorResolver {
    pub async fn start(&self) -> anyhow::Result<()> {
        info!("starting monitor resolver...");
        self.exchange_monitor.start().await
    }
}
