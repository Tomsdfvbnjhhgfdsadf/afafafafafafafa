use crate::blockchain::Blockchain;
use crate::ProxyClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const GECKO_SUPPORT: [&'static str; 3] = ["polygon-pos", "binance-smart-chain", "ethereum"];

impl Blockchain {
    pub(crate) fn to_gecko(self) -> String {
        match self {
            Blockchain::Polygon => "polygon-pos".to_string(),
            Blockchain::Bsc => "binance-smart-chain".to_string(),
            Blockchain::Eth => "ethereum".to_string(),
        }
    }

    pub(crate) fn from_gecko(str: String) -> Self {
        match &*str {
            "polygon-pos" => Self::Polygon,
            "binance-smart-chain" => Self::Bsc,
            _ => Self::Eth,
        }
    }
}

pub type ListAll = Vec<List>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct List {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub platforms: HashMap<String, Option<String>>,
}

pub async fn list_all(proxy: Option<&str>) -> anyhow::Result<ListAll> {
    let client = reqwest::Client::create_with_proxy(proxy);

    let req = client
        .get("https://api.coingecko.com/api/v3/coins/list?include_platform=true")
        .send()
        .await?
        .json::<ListAll>()
        .await?;
    Ok(req)
}

pub type Markets = Vec<Market>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    #[serde(rename = "current_price")]
    pub current_price: f64,
    #[serde(rename = "market_cap")]
    pub market_cap: f64,
    #[serde(rename = "market_cap_rank")]
    pub market_cap_rank: i64,
    #[serde(rename = "circulating_supply")]
    pub circulating_supply: Option<f64>,
    #[serde(rename = "total_supply")]
    pub total_supply: Option<f64>,
    #[serde(rename = "max_supply")]
    pub max_supply: Option<f64>,
    pub roi: Option<Roi>,
    #[serde(rename = "last_updated")]
    pub last_updated: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Roi {
    pub times: f64,
    pub currency: String,
    pub percentage: f64,
}

pub async fn markets(proxy: Option<&str>, page: Option<i32>) -> anyhow::Result<Markets> {
    let client = reqwest::Client::create_with_proxy(proxy);

    let req = client.get(format!("https://api.coingecko.com/api/v3/coins/markets?vs_currency=USD&order=market_cap_desc&per_page=250&page={}&sparkline=false", page.unwrap_or(1).to_string()))
        .send()
        .await?
        .json::<Markets>()
        .await?;
    Ok(req)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: String,
    pub symbol: String,
    pub name: String,
    #[serde(rename = "detail_platforms")]
    pub detail_platforms: HashMap<String, DetailPlatforms>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailPlatforms {
    #[serde(rename = "decimal_place")]
    pub decimal_place: Option<i64>,
    #[serde(rename = "contract_address")]
    pub contract_address: String,
}

impl Contract {
    pub(crate) fn to_model(self) -> Vec<crate::db::coingecko::gecko_chain::Model> {
        let mut result = Vec::with_capacity(self.detail_platforms.len());
        for i in self.detail_platforms {
            if GECKO_SUPPORT.contains(&&*i.0) {
                result.push(crate::db::coingecko::gecko_chain::Model {
                    id: 0,
                    gecko_id: self.id.clone(),
                    contract: i.1.contract_address,
                    decimals: i.1.decimal_place.unwrap_or(18) as i32,
                    blockchain: i.0.clone(),
                })
            }
        }

        result
    }
}

pub async fn info_contract(
    contract: impl Into<String>,
    blockchain: Blockchain,
    proxy: Option<&str>,
) -> anyhow::Result<Contract> {
    let client = reqwest::Client::create_with_proxy(proxy);

    let req = client
        .get(format!(
            "https://api.coingecko.com/api/v3/coins/{}/contract/{}",
            blockchain.to_gecko(),
            contract.into()
        ))
        .send()
        .await?
        .json::<Contract>()
        .await?;

    Ok(req)
}

pub async fn coin_by_id(id: impl Into<String>, proxy: Option<&str>) -> anyhow::Result<Contract> {
    let client = reqwest::Client::create_with_proxy(proxy);
    let id = id.into();

    let req = client
        .get(format!("https://api.coingecko.com/api/v3/coins/{}", id))
        .send()
        .await?
        .json::<Contract>()
        .await?;

    Ok(req)
}

#[cfg(test)]
mod tests {
    use crate::blockchain::Blockchain;

    #[tokio::test]
    async fn list_all() {
        let c = crate::db::coingecko::api::list_all(None).await.unwrap();
    }

    #[tokio::test]
    async fn markets() {
        let c = crate::db::coingecko::api::markets(None, None)
            .await
            .unwrap();
        println!("{:?}", c);
    }

    #[tokio::test]
    async fn info_contract() {
        let c = crate::db::coingecko::api::info_contract(
            "0xf4d2888d29D722226FafA5d9B24F9164c092421E",
            Blockchain::Eth,
            None,
        )
        .await
        .unwrap();
        println!("{:?}", c);
    }

    #[tokio::test]
    async fn coin_by_id() {
        let c = crate::db::coingecko::api::coin_by_id("staked-ether", None)
            .await
            .unwrap();
        println!("{:?}", c);
    }
}
