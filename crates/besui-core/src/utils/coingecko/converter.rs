use crate::domain::token::{Token, TokenId, TokenMarketData, TokenName, TokenSymbol};
use chrono::Utc;
use coingecko::response::coins::CoinsMarketItem;

pub struct CoingeckoConverter {}

impl CoingeckoConverter {
    pub fn coins_market_to_token(item: &CoinsMarketItem) -> anyhow::Result<Token> {
        let id = TokenId::try_from(item.id.clone())?;
        let symbol = TokenSymbol::try_from(item.symbol.clone())?;
        let name = TokenName::try_from(item.name.clone())?;

        let token_market_data = TokenMarketData {
            current_price: item.current_price,
            volume_24h: None,
            high_24h: item.high24_h,
            low_24h: item.low24_h,
        };

        Ok(Token {
            id,
            symbol,
            name,
            image: Some(item.image.clone()),
            last_updated: Utc::now(),
            circulating_supply: item.circulating_supply,
            total_supply: item.total_supply,
            max_supply: item.max_supply,
            market_data: Some(token_market_data),
        })
    }
}
