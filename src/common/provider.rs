use std::sync::Arc;

use super::{chain::Chain, types::HttpProvider};
use crate::{config::get_global_config, errors::Error};
use anyhow::Result;
use ethers::prelude::*;

#[derive(Debug)]
pub struct ProviderController {
    rpc_urls: Vec<String>,
    current_index: usize,
}

impl ProviderController {
    pub fn new(chain: Chain) -> Result<ProviderController> {
        let config = get_global_config();
        let chain_id: u64 = chain.into();
        let list_rpc_url: Option<Vec<String>> = config.chains.iter().find_map(|chain_config| {
            if chain_config.chain_id == chain_id {
                if chain_config.rpc_url.len() == 0 {
                    return None;
                }
                return Some(chain_config.rpc_url.clone());
            }
            return None;
        });
        if let Some(urls) = list_rpc_url {
            // filters invalid urls
            let valid_urls = urls
                .into_iter()
                .filter(|url| match Provider::<Http>::try_from(url) {
                    Ok(_) => return true,
                    Err(_) => return false,
                })
                .collect::<Vec<String>>();

            Ok(ProviderController {
                rpc_urls: valid_urls,
                current_index: 0,
            })
        } else {
            return Err(Error::NoDefaultProvider(chain_id).into());
        }
    }

    pub fn current_http_provider(&self) -> Result<HttpProvider> {
        let current_rpc_url = match self.rpc_urls.get(self.current_index) {
            Some(url) => url,
            None => self.rpc_urls.get(0).unwrap(),
        };

        let provider = Provider::<Http>::try_from(current_rpc_url)?;

        Ok(provider)
    }

    pub fn next_http_provider(&mut self) -> Result<HttpProvider> {
        self.current_index += 1;
        let rpc_url = match self.rpc_urls.get(self.current_index) {
            Some(rpc_url) => rpc_url,
            None => return Err(Error::ProviderNotFound.into()),
        };
        let provider = match Provider::<Http>::try_from(rpc_url) {
            Ok(provider) => Some(provider),
            Err(_) => None,
        };

        if provider.is_none() {
            return Err(Error::ProviderNotFound.into());
        }
        let provider = provider.unwrap();
        if self.current_index >= self.rpc_urls.len() - 1 {
            self.current_index = 0;
        }
        return Ok(provider);
    }
}

#[cfg(test)]
mod test_provider_controller {
    use ethers::providers::Middleware;

    use super::ProviderController;
    use crate::{common::chain::Chain, errors::Error};
    use anyhow::Result;
    use ethers::prelude::*;

    #[test]
    fn test_new_provider_controller() {
        let chain = Chain::Mainnet;
        let provider_controller = ProviderController::new(chain);
        assert!(provider_controller.is_ok());

        let chain: Chain = Chain::Kava;
        let provider_controller = ProviderController::new(chain);
        assert_eq!(provider_controller.is_err(), true);
        let err = provider_controller.unwrap_err();
        let chain_id: u64 = chain.into();
        let expected_err = Error::NoDefaultProvider(chain_id);
        assert_eq!(err.to_string(), expected_err.to_string());
    }

    #[tokio::test]
    async fn test_next_http_provider() -> Result<()> {
        let chain = Chain::Mainnet;
        let provider_controller = ProviderController::new(chain);
        assert!(provider_controller.is_ok());

        let mut provider_controller = provider_controller.unwrap();
        let provider = provider_controller.next_http_provider().unwrap();

        let block_number = 13352265;
        let block_1 = provider.get_block(block_number).await?.unwrap();

        let provider = provider_controller.next_http_provider().unwrap();
        let block_2 = provider.get_block(block_number).await?.unwrap();

        assert_eq!(block_1.number, block_2.number);
        Ok(())
    }
}
