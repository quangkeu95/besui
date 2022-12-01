use crate::errors::Error;
use chrono::{DateTime, Utc};
use derive_more::{Display, Into};
use getset::Setters;
use serde::{Deserialize, Serialize};

/**
 * A token ID, must not be empty
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display, Into)]
pub struct TokenId(String);

impl TryFrom<String> for TokenId {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.is_empty() {
            Err(Error::TokenIdCannotBeEmpty)
        } else {
            Ok(Self(s))
        }
    }
}

impl<'a> TryFrom<&'a str> for TokenId {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Self::try_from(s.to_owned())
    }
}

/**
 * A token symbol, must not be empty
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display, Into)]
pub struct TokenSymbol(String);

impl TryFrom<String> for TokenSymbol {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.is_empty() {
            Err(Error::TokenSymbolCannotBeEmpty)
        } else {
            Ok(Self(s))
        }
    }
}

impl<'a> TryFrom<&'a str> for TokenSymbol {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Self::try_from(s.to_owned())
    }
}

/**
 * A token name, must not be empty
 */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display, Into)]
pub struct TokenName(String);

impl TryFrom<String> for TokenName {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.is_empty() {
            Err(Error::TokenNameCannotBeEmpty)
        } else {
            Ok(Self(s))
        }
    }
}

impl<'a> TryFrom<&'a str> for TokenName {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Self::try_from(s.to_owned())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenMarketData {
    pub current_price: Option<f64>,
    pub volume_24h: Option<f64>,
    pub high_24h: Option<f64>,
    pub low_24h: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Setters, PartialEq)]
pub struct Token {
    pub id: TokenId,
    pub symbol: TokenSymbol,
    pub name: TokenName,
    pub updated_at: DateTime<Utc>,
    #[getset(set)]
    pub image: Option<String>,
    #[getset(set)]
    pub circulating_supply: Option<i64>,
    #[getset(set)]
    pub total_supply: Option<i64>,
    #[getset(set)]
    pub max_supply: Option<i64>,
    #[getset(set)]
    pub market_data: Option<TokenMarketData>,
}

impl Token {
    pub fn new(id: &str, symbol: &str, name: &str) -> anyhow::Result<Self> {
        Ok(Token {
            id: TokenId::try_from(id)?,
            symbol: TokenSymbol::try_from(symbol)?,
            name: TokenName::try_from(name)?,
            updated_at: Utc::now(),
            image: None,
            circulating_supply: None,
            total_supply: None,
            max_supply: None,
            market_data: None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenWithPair {
    pub id: TokenId,
    pub pair: Vec<Pair>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pair {
    pub base: TokenId,
    pub quote: TokenId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_id_must_not_be_empty() {}
}
