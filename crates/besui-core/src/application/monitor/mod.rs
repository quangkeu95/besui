// use tracing::info;

// use crate::resolver::RootResolver;

// use self::exchange_monitor::ExchangeMonitor;
// use self::token_monitor::TokenMonitor;
// use std::sync::Arc;

// pub mod exchange_monitor;
// pub mod token_monitor;

// pub struct MonitorResolver {
//     exchange_monitor: Arc<ExchangeMonitor>,
//     token_monitor: Arc<TokenMonitor>,
// }

// impl MonitorResolver {
//     pub fn new() -> Self {
//         MonitorResolver {
//             exchange_monitor: Arc::new(ExchangeMonitor::new()),
//             token_monitor: Arc::new(TokenMonitor::new()),
//         }
//     }

//     pub async fn start(&self) -> anyhow::Result<()> {
//         info!("Starting monitor resolver...");
//         self.exchange_monitor.start().await
//     }
// }
