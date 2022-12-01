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

        let circulating_supply = Self::f64_to_i64(item.circulating_supply);
        let total_supply = Self::f64_to_i64(item.total_supply);
        let max_supply = Self::f64_to_i64(item.max_supply);

        Ok(Token {
            id,
            symbol,
            name,
            image: Some(item.image.clone()),
            updated_at: Utc::now(),
            circulating_supply,
            total_supply,
            max_supply,
            market_data: Some(token_market_data),
        })
    }

    fn f64_to_i64(input: Option<f64>) -> Option<i64> {
        match input {
            Some(val) => Some(val.round() as i64),
            None => None,
        }
    }
}
