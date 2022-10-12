use crate::common::{chain::Chain, provider::ProviderController, types::HttpProvider};
use crate::config::get_global_config;
use crate::errors::Error;
use anyhow::Result;
use ethers::prelude::*;
use tokio::sync::broadcast::Sender;
use tracing::{debug, error};

pub mod types;

const BLOCK_RANGE: u64 = 1000;

pub async fn get_lastest_block(
    chain: Chain,
    custom_provider: Option<HttpProvider>,
) -> Result<Block<H256>> {
    let provider = wrapper_load_provider(chain, custom_provider)?;
    let block = provider.get_block(BlockNumber::Latest).await?.unwrap();
    Ok(block)
}

pub async fn get_block(
    block_number: u64,
    chain: Chain,
    custom_provider: Option<HttpProvider>,
) -> Result<Block<H256>> {
    let provider = wrapper_load_provider(chain, custom_provider)?;
    let block = provider.get_block(block_number).await?.unwrap();
    Ok(block)
}

fn wrapper_load_provider(
    chain: Chain,
    custom_provider: Option<HttpProvider>,
) -> Result<HttpProvider> {
    let config = get_global_config();
    let provider = match custom_provider {
        Some(p) => p,
        None => {
            let mut provider_controller = ProviderController::new(chain, config.chains.clone())?;
            let provider = provider_controller.next_http_provider()?;
            provider
        }
    };
    Ok(provider)
}

pub async fn wrapper_get_events(
    from_block: u64,
    to_block: u64,
    filter: Filter,
    chain: Chain,
    sender: Sender<Vec<Log>>,
    custom_provider: Option<HttpProvider>,
) -> Result<()> {
    let mut start_block = from_block;

    let provider = wrapper_load_provider(chain, custom_provider)?;
    loop {
        if start_block > to_block {
            break;
        }
        let mut next_block = start_block + BLOCK_RANGE;
        if next_block > to_block {
            next_block = to_block;
        }

        let internal_filter = filter.clone().from_block(start_block).to_block(next_block);
        let logs = match provider.get_logs(&internal_filter).await {
            Ok(logs) => logs,
            Err(err) => {
                debug!("error: {:?}", err);
                return Err(err.into());
            }
        };
        if logs.len() > 0 {
            match sender.send(logs) {
                Ok(_) => (),
                Err(error) => {
                    error!("receiver dropped error {:?}", error);
                    return Err(Error::ReceiverDroppedError.into());
                }
            }
        }

        start_block = next_block + 1;
    }

    Ok(())
}

#[cfg(test)]
mod test_fetcher {
    use super::{get_block, get_lastest_block, wrapper_get_events};
    use crate::common::{chain::Chain, types::TRANSFER_TOPIC};
    use anyhow::Result;
    use ethers::prelude::*;
    use ethers::utils::keccak256;
    use test_log::test;
    use tokio::sync::broadcast::{self, error::RecvError};
    use tracing::info;

    #[tokio::test]
    async fn test_get_latest_block() {
        let chain = Chain::Mainnet;
        // use default provider
        let latest_block = get_lastest_block(chain, None).await;
        assert!(latest_block.is_ok());
        let latest_block = latest_block.unwrap();
        assert!(latest_block.number.is_some());
    }

    #[tokio::test]
    async fn test_get_block() {
        let chain = Chain::Mainnet;
        let block_number: u64 = 15594327;
        let block = get_block(block_number, chain, None).await;
        assert!(block.is_ok());
        let block = block.unwrap();
        assert!(block.number.is_some());
    }

    #[test(tokio::test)]
    async fn test_wrapper_get_events() -> Result<()> {
        let from_block: u64 = 13332265;
        let to_block: u64 = 13352265;
        let from_address: Address = "0x09b64e3d589ae90acce69c75c346722d8ebfb65d"
            .parse()
            .unwrap();
        let chain = Chain::Mainnet;
        let filter: Filter = Filter::new()
            .topic0(ValueOrArray::Value(H256::from(keccak256(TRANSFER_TOPIC))))
            .topic1(ValueOrArray::Value(H256::from(from_address)));

        let (tx, mut rx) = broadcast::channel(20);
        tokio::spawn(async move {
            let _ = wrapper_get_events(from_block, to_block, filter, chain, tx, None).await;
        });

        loop {
            match rx.recv().await {
                Ok(logs) => info!("logs: {:?}", logs),
                Err(error) => {
                    if error == RecvError::Closed {
                        info!("channel closed");
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}
