use crate::common::{chain::Chain, provider::ProviderController, types::HttpProvider};
use anyhow::Result;
use ethers::prelude::*;
use futures::Future;
use tracing::info;

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
    let provider = match custom_provider {
        Some(p) => p,
        None => {
            let mut provider_controller = ProviderController::new(chain)?;
            let provider = provider_controller.next_http_provider()?;
            provider
        }
    };
    Ok(provider)
}

pub async fn wrapper_get_events<F, Fut>(
    from_block: u64,
    to_block: u64,
    filter: Filter,
    chain: Chain,
    custom_provider: Option<HttpProvider>,
    mut handler: F,
) -> Result<()>
where
    F: FnMut(Vec<Log>) -> Fut,
    Fut: Future<Output = ()>,
{
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
        println!("start_block: {:?} end_block: {:?}", start_block, next_block);

        let internal_filter = filter.clone().from_block(start_block).to_block(next_block);
        let logs = match provider.get_logs(&internal_filter).await {
            Ok(logs) => logs,
            Err(err) => {
                println!("error: {:?}", err);
                return Err(err.into());
            }
        };

        println!("logs length: {:?}", logs.len());
        // logs.iter().for_each(|log| {
        //     println!("{:?}", log);
        // });

        // handler(logs).await;
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
    use std::str::FromStr;

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

    #[tokio::test]
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
        let handler = |arr_logs: Vec<Log>| async move {
            arr_logs.iter().for_each(|log| {
                println!("{:?}", log);
            });
        };

        // let provider = get_default_provider(chain)?;
        // let logs = provider.get_logs(&filter).await?;
        // println!("{:?}", logs);

        // assert!(!logs.is_empty());
        let _ = wrapper_get_events(from_block, to_block, filter, chain, None, handler).await;
        Ok(())
    }
}
