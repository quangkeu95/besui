use anyhow::Context;
use coingecko::CoinGeckoClient;

use crate::domain::{exchange::Exchange, token::Token};

use self::{converter::CoingeckoConverter, filter::CoinsMarketsFilter};

pub mod converter;
pub(self) mod dto;
pub mod filter;
pub struct CoingeckoClient {
    client: CoinGeckoClient,
}

impl Default for CoingeckoClient {
    fn default() -> Self {
        CoingeckoClient {
            client: CoinGeckoClient::default(),
        }
    }
}

impl CoingeckoClient {
    pub async fn get_all_token_ids(&self) -> anyhow::Result<Vec<dto::TokenId>> {
        let coins = self
            .client
            .coins_list(false)
            .await
            .context("error call api /coins/list from coingecko")?;
        let coin_ids = coins
            .into_iter()
            .map(|coin| dto::TokenId {
                id: coin.id,
                symbol: coin.symbol,
                name: coin.name,
            })
            .collect::<Vec<dto::TokenId>>();
        Ok(coin_ids)
    }

    pub async fn get_coins_markets(
        &self,
        filter: CoinsMarketsFilter,
    ) -> anyhow::Result<Vec<Token>> {
        let currency = filter.currency.as_str();
        let ids = filter.get_ids();
        let category = filter.get_category();
        let order = filter.get_order();
        let coins = self
            .client
            .coins_markets(
                currency,
                &ids,
                category,
                order,
                filter.per_page,
                filter.page,
                filter.sparkline,
                &filter.price_change_percentage,
            )
            .await?;
        let result = coins
            .iter()
            .map(|item| {
                let token = CoingeckoConverter::coins_market_to_token(item).unwrap();
                token
            })
            .collect::<Vec<Token>>();
        Ok(result)
    }

    pub async fn get_list_exchanges(&self) -> anyhow::Result<Vec<Exchange>> {
        let resp = self.client.exchanges_list().await?;
        let exchanges = resp
            .into_iter()
            .map(|item| Exchange {
                id: item.id.clone(),
                name: item.name.clone(),
            })
            .collect::<Vec<Exchange>>();
        Ok(exchanges)
    }
}

#[cfg(test)]
mod coingecko_client_test {
    use super::filter::*;
    use super::*;
    use crate::utils::output::TableOutput;
    use claims::assert_ok;
    use coingecko::params::{MarketsOrder, PriceChangePercentage};

    #[tokio::test]
    async fn test_get_all_token_ids() {
        let client = CoingeckoClient::default();
        let token_ids_result = client.get_all_token_ids().await;
        let token_ids = assert_ok!(token_ids_result);
        assert!(token_ids.len() > 0);
        println!("total tokens: {}", token_ids.len());
    }

    #[tokio::test]
    async fn test_get_coins_markets() {
        let client = CoingeckoClient::default();
        let coins_markets_filter_result = CoinsMarketsFilterBuilder::default()
            .currency("usd".to_string())
            .ids(vec![])
            .category(None)
            .order(MarketsOrder::GeckoDesc)
            .price_change_percentage(vec![PriceChangePercentage::TwentyFourHours])
            .build();
        let filter = assert_ok!(coins_markets_filter_result);
        let result = client.get_coins_markets(filter).await;
        let tokens = assert_ok!(result);
        assert!(tokens.len() > 0);
        assert_eq!(tokens.len(), 250);

        // let mut table = TableOutput::new(vec!["TokenID", "Symbol", "Name"]);

        // let rows = tokens
        //     .into_iter()
        //     .map(|item| {
        //         vec![
        //             item.id.clone().into(),
        //             item.symbol.clone().into(),
        //             item.name.clone().into(),
        //         ]
        //     })
        //     .collect::<Vec<Vec<String>>>();

        // table.add_rows(rows);

        // table.printstd();
    }

    #[tokio::test]
    async fn test_get_list_exchanges() {
        let client = CoingeckoClient::default();
        let result = client.get_list_exchanges().await;
        let exchanges = assert_ok!(result);
        assert!(exchanges.len() > 0);
    }
}
