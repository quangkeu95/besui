use crate::errors::Error;
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

#[cfg(test)]
mod test_chain_common {
    use super::{get_chain, Chain};
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
}
