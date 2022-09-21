use crate::{config::get_global_config, errors::Error};
use anyhow::Result;
use ethers::prelude::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(IntoPrimitive, TryFromPrimitive, PartialEq, Debug, Clone, Copy)]
#[repr(u64)]
pub enum Chain {
    Mainnet = 1,
    Ropsten = 3,
    Bsc = 56,
    BscTestnet = 97,
    Avalanche = 43114,
    AvalancheFuji = 43113,
    Polygon = 137,
    Mumbai = 80001,
    ArbitrumOne = 42161,
    ArbitrumNova = 42170,
    ArbitrumRinkeby = 421611,
    Optimism = 10,
    OptimismKovan = 69,
    Kava = 2222,
}

pub fn get_chain(chain_id: u64) -> Result<Chain> {
    match Chain::try_from(chain_id) {
        Ok(chain) => Ok(chain),
        Err(_) => Err(Error::ChainNotSupported(chain_id).into()),
    }
}

pub fn get_default_provider(chain: Chain) -> Result<Provider<Http>> {
    let config = get_global_config();
    let chain_id: u64 = chain.into();
    let provider: Option<Provider<Http>> = config.chains.iter().find_map(|chain_config| {
        if chain_config.chain_id == chain_id {
            let provider = match Provider::<Http>::try_from(chain_config.rpc_url.clone()) {
                Ok(provider) => Some(provider),
                Err(_) => None,
            };
            return provider;
        }
        return None;
    });

    if let Some(provider) = provider {
        return Ok(provider);
    }
    return Err(Error::NoDefaultProvider(chain_id).into());
}

#[cfg(test)]
mod test_chain_common {
    use super::{get_chain, get_default_provider, Chain};
    use crate::errors::Error;

    #[test]
    fn test_chain_id_converter() {
        let mainnet_id: u64 = Chain::Mainnet.into();
        assert_eq!(mainnet_id, 1);
        let ropsten_id: u64 = Chain::Ropsten.into();
        assert_eq!(ropsten_id, 3);
    }

    #[test]
    fn test_get_chain() {
        let chain_id: u64 = 1;
        let chain = get_chain(chain_id);
        assert_eq!(chain.is_ok(), true);
        assert_eq!(chain.unwrap(), Chain::Mainnet);

        let chain_id: u64 = 0;
        let chain = get_chain(chain_id);
        assert_eq!(chain.is_err(), true);
        let err = chain.unwrap_err();
        let expected_err = Error::ChainNotSupported(0);

        assert_eq!(err.to_string(), expected_err.to_string());
    }

    #[test]
    fn test_get_default_provider() {
        let chain: Chain = Chain::Mainnet;
        let provider = get_default_provider(chain);
        assert_eq!(provider.is_ok(), true);

        let chain: Chain = Chain::Kava;
        let provider = get_default_provider(chain);
        assert_eq!(provider.is_err(), true);
        let err = provider.unwrap_err();
        let chain_id: u64 = chain.into();
        let expected_err = Error::NoDefaultProvider(chain_id);
        assert_eq!(err.to_string(), expected_err.to_string());
    }
}
