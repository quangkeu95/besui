use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::broadcast::Receiver;

use crate::common::chain::get_chain;

use self::on_chain::TransferWatcher;

pub mod on_chain;

pub async fn watch_transfer(chain_id: u64, token_address: &str, wallet_address: &str) {}

pub struct TransferTx {}

#[async_trait]
pub trait TransferWatcherTrait {
    async fn watch_transfer(
        &self,
        chain_id: u64,
        token_address: &str,
        wallet_address: &str,
    ) -> Result<Receiver<TransferTx>>;
}

pub struct OnChainTransferWatcher {}

// #[async_trait]
// impl TransferWatcherTrait for OnChainTransferWatcher {
//     async fn watch_transfer(
//         &self,
//         chain_id: u64,
//         token_address: &str,
//         wallet_address: &str,
//     ) -> Result<Receiver<TransferTx>> {
//         let chain_id = get_chain(chain_id)?;

//     }
// }

// fn new_transfer_watcher() -> Box<dyn TransferWatcherTrait> {}
