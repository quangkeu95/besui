use coingecko::params::{MarketsOrder, PriceChangePercentage};
use derive_builder::Builder;

const SUPPORTED_CURRENCY: [&'static str; 3] = ["usd", "eur", "jpy"];

#[derive(Builder)]
#[builder(build_fn(validate = "Self::validate"))]
#[builder(pattern = "owned")]
pub struct CoinsMarketsFilter {
    pub currency: String,
    pub ids: Vec<String>,
    pub category: Option<String>,
    pub order: MarketsOrder,
    #[builder(default = "250")]
    pub per_page: i64,
    #[builder(default = "1")]
    pub page: i64,
    #[builder(default = "false")]
    pub sparkline: bool,
    pub price_change_percentage: Vec<PriceChangePercentage>,
}

impl CoinsMarketsFilterBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(currency) = &self.currency {
            if !SUPPORTED_CURRENCY.contains(&currency.as_str()) {
                return Err("currency not supported".to_string());
            }
        }
        Ok(())
    }
}

impl CoinsMarketsFilter {
    pub fn get_ids(&self) -> Vec<&str> {
        self.ids
            .iter()
            .map(|item| item.as_str())
            .collect::<Vec<&str>>()
    }

    pub fn get_category(&self) -> Option<&str> {
        match &self.category {
            Some(c) => Some(c.as_str()),
            None => None,
        }
    }

    pub fn get_order(&self) -> MarketsOrder {
        match &self.order {
            MarketsOrder::MarketCapDesc => MarketsOrder::MarketCapDesc,
            MarketsOrder::MarketCapAsc => MarketsOrder::MarketCapDesc,
            MarketsOrder::GeckoDesc => MarketsOrder::GeckoDesc,
            MarketsOrder::GeckoAsc => MarketsOrder::GeckoAsc,
            MarketsOrder::VolumeDesc => MarketsOrder::VolumeDesc,
            MarketsOrder::VolumeAsc => MarketsOrder::VolumeAsc,
            MarketsOrder::IdDesc => MarketsOrder::IdDesc,
            MarketsOrder::IdAsc => MarketsOrder::IdAsc,
        }
    }
}

#[cfg(test)]
mod coins_markets_filter_tests {
    use super::*;
    use claims::assert_ok;
    use coingecko::params::{MarketsOrder, PriceChangePercentage};

    #[test]
    fn test_coins_markets_filter_builder() {
        let coins_markets_filter_result = CoinsMarketsFilterBuilder::default()
            .currency("usd".to_string())
            .ids(vec![])
            .category(None)
            .order(MarketsOrder::GeckoDesc)
            .price_change_percentage(vec![PriceChangePercentage::TwentyFourHours])
            .build();
        let filter = assert_ok!(coins_markets_filter_result);
    }

    #[test]
    fn test_get_ids() {
        let coins_markets_filter_result = CoinsMarketsFilterBuilder::default()
            .currency("usd".to_string())
            .ids(vec!["btc".to_string()])
            .category(None)
            .order(MarketsOrder::GeckoDesc)
            .price_change_percentage(vec![PriceChangePercentage::TwentyFourHours])
            .build();

        let filter = assert_ok!(coins_markets_filter_result);
        let ids = filter.get_ids();
        assert!(ids.len() > 0);
        assert_eq!(ids, vec!["btc"]);
    }
}
