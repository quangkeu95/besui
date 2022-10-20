use coingecko::CoinGeckoClient;

pub fn new_coingecko_client() -> CoinGeckoClient {
    CoinGeckoClient::default()
}
