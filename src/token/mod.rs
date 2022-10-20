use crate::common::coingecko::new_coingecko_client;
use anyhow::Result;

#[derive(Debug)]
pub struct TokenInfo {
    pub token_id: String,
    pub token_symbol: String,
    pub total_supply: TokenTotalSupply,
}

#[derive(Debug)]
pub enum TokenTotalSupply {
    Infinite,
    Finite(u64),
}

pub async fn get_token_info(token_id: &'static str) {}

pub struct TokenIdInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

pub async fn get_list_token_ids() -> Result<Vec<TokenIdInfo>> {
    let client = new_coingecko_client();
    let coin_list = client.coins_list(false).await?;
    let result = coin_list
        .into_iter()
        .map(|item| TokenIdInfo {
            id: item.id,
            symbol: item.symbol,
            name: item.name,
        })
        .collect::<Vec<TokenIdInfo>>();
    Ok(result)
}
