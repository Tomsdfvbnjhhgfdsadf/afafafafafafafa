use crate::blockchain::{Blockchain, BSC_RPC, ETH_RPC, POLYGON_RPC};
use crate::bsc::BscStables;
use crate::polygon::checker_abi::CHECKER_ABI;
use crate::{db, telegram, CONFIG};
use anyhow::{anyhow, bail};
use async_trait::async_trait;
use ethers::prelude::{Http, JsonRpcClient, Provider, H160};
use ethers::types::{Address, U256};
use itertools::Itertools;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

lazy_static! {
    pub static ref PROXY: Vec<String> = {
        let file = fs::read_to_string("proxy.txt").unwrap();
        file.split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    };
}

lazy_static! {
    static ref POLYGON_CHECKER_CONTRACT: Address =
        Address::from_str("0x2352c63A83f9Fd126af8676146721Fa00924d7e4").unwrap();
    static ref ETH_CHECKER_CONTRACT: Address =
        Address::from_str("0xb1f8e55c7f64d203c1400b9d8555d050f94adf39").unwrap();
    static ref BSC_CHECKER_CONTRACT: Address =
        Address::from_str("0x2352c63A83f9Fd126af8676146721Fa00924d7e4").unwrap();
}

trait BalanceConvert {
    fn to_f64_with_decimals(self, decimals: u32) -> f64;
}

impl BalanceConvert for U256 {
    fn to_f64_with_decimals(self, decimals: u32) -> f64 {
        let devided = U256::from(10u64.pow(decimals));
        let (quotient, remainder) = self.div_mod(devided);
        let remainder_fractional = (remainder.as_u128() as f64) * 10.0f64.powf(-(decimals as f64));
        quotient.as_u128() as f64 + remainder_fractional
    }
}

type TopContracts = Vec<(
    db::coingecko::gecko_chain::Model,
    Vec<db::coingecko::gecko_list::Model>,
)>;

trait ToContracts {
    fn to_contract_list(self) -> Vec<Address>;
}

impl ToContracts for TopContracts {
    fn to_contract_list(self) -> Vec<Address> {
        let mut result = Vec::with_capacity(self.len());
        for i in self.into_iter() {
            let i = Address::from_str(&*i.0.contract);
            match i {
                Ok(v) => result.push(v),
                Err(_) => continue,
            }
        }

        result
    }
}

impl Blockchain {
    pub(crate) fn get_checker_contract(self) -> Address {
        match self {
            Blockchain::Polygon => *POLYGON_CHECKER_CONTRACT,
            Blockchain::Bsc => *BSC_CHECKER_CONTRACT,
            Blockchain::Eth => *ETH_CHECKER_CONTRACT,
        }
    }

    pub(crate) async fn get_checker_abi_contract(
        self,
        client: Option<Arc<Provider<Http>>>,
    ) -> Arc<CHECKER_ABI<Provider<Http>>> {
        let contract_address = self.get_checker_contract();
        let client = match client {
            None => self.get_random_http_rpc().await,
            Some(v) => v.into(),
        };

        Arc::new(CHECKER_ABI::new(contract_address, client))
    }

    pub(crate) async fn get_top_contracts(self) -> anyhow::Result<TopContracts> {
        Ok(db::coingecko::coingecko::find_by_blockchain(self).await?)
    }

    pub(crate) fn get_default_gecko_id(self) -> String {
        match self {
            Blockchain::Polygon => String::from("matic-network"),
            Blockchain::Bsc => String::from("binancecoin"),
            Blockchain::Eth => String::from("ethereum"),
        }
    }
}

// impl Blockchain {
//     async fn get_random_checker_http_rpc(self) -> Arc<Provider<Http>> {
//         match self {
//             Blockchain::Polygon => {
//                 let connections = POLYGON_RPC
//                     .get()
//                     .await
//                     .iter()
//                     .filter(|x| x.http.url().to_string().contains("nodereal"))
//                     .collect::<Vec<_>>();
//                 let i = fastrand::usize(..connections.len());
//                 connections[i].http.clone()
//             }
//             Blockchain::Bsc => {
//                 let connections = BSC_RPC
//                     .get()
//                     .await
//                     .iter()
//                     .filter(|x| x.http.url().to_string().contains("nodereal"))
//                     .collect::<Vec<_>>();
//                 let i = fastrand::usize(..connections.len());
//                 connections[i].http.clone()
//             }
//             Blockchain::Eth => {
//                 let connections = ETH_RPC
//                     .get()
//                     .await
//                     .iter()
//                     .filter(|x| x.http.url().to_string().contains("nodereal"))
//                     .collect::<Vec<_>>();
//                 let i = fastrand::usize(..connections.len());
//                 connections[i].http.clone()
//             }
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct Balances {
    pub address: Address,
    pub sum: f64,
    pub balances: Vec<Balance>,
}

#[derive(Debug, Clone)]
pub struct Balance {
    pub symbol: String,
    pub sum: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
struct TokenBalance {
    address: Address,
    #[serde(rename = "tokenBalances", default)]
    token_balances: Vec<TokenBalances>,
    #[serde(rename = "pageKey", default)]
    page_key: Option<String>,
}

impl TokenBalance {
    pub(crate) async fn to_prices(
        self,
        blockchain: Blockchain,
        contracts: TopContracts,
    ) -> Balances {
        let mut balances_usd = Balances {
            address: self.address,
            sum: 0.0,
            balances: Vec::with_capacity(self.token_balances.len()),
        };

        for balances in self.token_balances {
            let balance = match balances.token_balance {
                None => continue,
                Some(v) => v,
            };

            if balances.contract_address == Address::from([0; 20]) {
                let balance = balance.to_f64_with_decimals(18);
                let gecko_data =
                    match db::coingecko::coingecko::find_by_name(blockchain.get_default_gecko_id())
                        .await
                    {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                let usd_sum = balance * gecko_data.price;
                balances_usd.sum += usd_sum;

                balances_usd.balances.push(Balance {
                    symbol: gecko_data.symbol.clone(),
                    sum: usd_sum,
                });

                continue;
            }

            let contract = contracts
                .iter()
                .filter(|x| {
                    x.0.contract
                        .eq(&("0x".to_owned() + &hex::encode(balances.contract_address)))
                })
                .collect::<Vec<_>>();

            let contract = match contract.get(0) {
                None => continue,
                Some(v) => v,
            };
            let gecko_data = match contract.1.get(0) {
                None => continue,
                Some(v) => v,
            };

            let balance = balance.to_f64_with_decimals(contract.0.decimals as u32);
            let usd_sum = balance * gecko_data.price;

            balances_usd.sum += usd_sum;
            balances_usd.balances.push(Balance {
                symbol: gecko_data.symbol.clone(),
                sum: usd_sum,
            })
        }

        balances_usd
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, PartialEq, Eq)]
struct TokenBalances {
    #[serde(rename = "contractAddress", default)]
    contract_address: Address,
    #[serde(
        rename = "tokenBalance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    token_balance: Option<U256>,
}

// #[async_trait]
// trait CheckerProvider: Sync + Send + Debug {
//     async fn get_token_balance<T: Into<Address> + Send + Sync>(
//         self: Arc<Self>,
//         address: T,
//         blockchain: Blockchain,
//     ) -> anyhow::Result<TokenBalance>;
// }

#[async_trait]
trait CheckerHttp: Sync + Send + Debug {
    async fn get_token_balance_contract(
        self: Arc<Self>,
        address_list: Vec<String>,
        contracts_list: Vec<Address>,
        blockchain: Blockchain,
    ) -> anyhow::Result<Vec<TokenBalance>>;
}

// #[async_trait]
// impl<P: JsonRpcClient + 'static> CheckerProvider for Provider<P> {
//     async fn get_token_balance<T: Into<Address> + Send + Sync>(
//         self: Arc<Self>,
//         address: T,
//         blockchain: Blockchain,
//     ) -> anyhow::Result<TokenBalance> {
//         let address = address.into();
//         let balance = self.clone().get_balance(address, None).await?;
//         let mut data: TokenBalance = self.request("alchemy_getTokenBalances", [address]).await?;
//
//         let contract = match blockchain {
//             Blockchain::Polygon => {
//                 Address::from_str("0x0000000000000000000000000000000000001010").unwrap()
//             }
//             _ => Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
//         };
//
//         data.token_balances.push(TokenBalances {
//             contract_address: contract,
//             token_balance: Some(balance),
//         });
//
//         Ok(data)
//     }
// }

#[async_trait]
impl CheckerHttp for Provider<Http> {
    async fn get_token_balance_contract(
        self: Arc<Self>,
        address_list: Vec<String>,
        mut contracts_list: Vec<Address>,
        blockchain: Blockchain,
    ) -> anyhow::Result<Vec<TokenBalance>> {
        if blockchain != Blockchain::Polygon {
            contracts_list.push(Address::from([0; 20]));
        }

        let address_list = address_list
            .into_iter()
            .map(|x| Address::from_str(&*x).unwrap())
            .collect::<Vec<Address>>();
        let abi = blockchain
            .get_checker_abi_contract(Some(self.clone()))
            .await;

        let balances = abi
            .balances(address_list.clone(), contracts_list.clone())
            .call()
            .await?;

        let balances_wallet = balances.chunks(contracts_list.len()).collect::<Vec<_>>();

        let mut result = Vec::with_capacity(address_list.len());

        for balance_wallet in balances_wallet.into_iter().zip(address_list) {
            let mut token_balance = TokenBalance {
                address: balance_wallet.1,
                token_balances: vec![],
                page_key: None,
            };

            for balance in balance_wallet.0.into_iter().zip(contracts_list.clone()) {
                let balance_token = balance.0.clone();
                if balance_token > U256::from(0) {
                    token_balance.token_balances.push(TokenBalances {
                        contract_address: balance.1,
                        token_balance: Some(balance_token),
                    })
                }
            }

            result.push(token_balance)
        }

        Ok(result)
    }
}

pub async fn checker(print: bool) {
    const CHUNKS: usize = 25;

    loop {
        let all_wallets = match db::db_get_all().await {
            Ok(v) => v,
            Err(err) => {
                println!("Error: {} in db, retry", err);
                tokio::time::sleep(Duration::from_secs(30)).await;
                continue;
            }
        };

        let wallets_chuncks = all_wallets.chunks(CHUNKS).collect::<Vec<_>>();

        for wallets in wallets_chuncks {
            let wallets_string = wallets
                .into_iter()
                .map(|x| x.address.clone())
                .collect::<Vec<_>>();
            let balances = match get_token_balances(wallets_string).await {
                Ok(v) => v,
                Err(e) => {
                    println!("Error {:?}", e.to_string());
                    continue;
                }
            };

            tokio::spawn(check_and_notify(
                balances.clone(),
                wallets.to_vec().clone(),
                print,
            ));
        }

        tokio::time::sleep(Duration::from_secs(CONFIG.checker_sleep)).await;
    }
}

pub async fn check_and_notify(
    balances: Vec<Balances>,
    wallets: Vec<db::Model>,
    print: bool,
) -> anyhow::Result<()> {
    let mut result_vec = Vec::with_capacity(15);
    for (balance, wallet) in balances.chunks(3).zip(wallets) {
        let mut temp_bal = WalletBalance {
            wallet: wallet.clone(),
            full_sum: 0.0,
            balance_bool: false,
            valid_balances: vec![],
        };

        for value in balance {
            temp_bal.full_sum += value.sum;
            temp_bal.valid_balances.push(value.clone());
        }
        temp_bal.balance_bool = temp_bal.full_sum >= CONFIG.checker_min_sum.as_u128() as f64;

        result_vec.push(temp_bal);
    }

    for wallet_balance in result_vec {
        db::balances::push_or_change(wallet_balance.full_sum, wallet_balance.wallet.clone())
            .await
            .unwrap_or(());

        if print {
            println!(
                "wallet: {}, balance: ({}, {})",
                &wallet_balance.wallet.address,
                wallet_balance.full_sum,
                wallet_balance.balance_bool
            );
        }

        if wallet_balance.balance_bool {
            for i in &*CONFIG.checker_telegram_id {
                match telegram::send_msg(
                    None,
                    i.clone(),
                    wallet_balance.wallet.clone(),
                    wallet_balance.full_sum,
                )
                .await
                {
                    Ok(_) => continue,
                    Err(_) => tokio::spawn(spam(wallet_balance.wallet.address.clone())),
                };
            }
        }
    }

    Ok(())
}

// pub async fn checker(print: bool) {
//     loop {
//         let all_wallets = match db::db_get_all().await {
//             Ok(v) => v,
//             Err(err) => {
//                 println!("Error: {} in db, retry", err);
//                 continue;
//             }
//         };
//
//         let chunks = 10;
//         let mut polygon_rpc = Vec::with_capacity(chunks);
//         let mut bsc_rpc = Vec::with_capacity(chunks);
//         let mut eth_rpc = Vec::with_capacity(chunks);
//
//         for _ in 0..chunks {
//             polygon_rpc.push(Blockchain::Polygon.get_random_checker_http_rpc().await);
//             bsc_rpc.push(Blockchain::Bsc.get_random_checker_http_rpc().await);
//             eth_rpc.push(Blockchain::Eth.get_random_checker_http_rpc().await);
//         }
//
//         for wallets in all_wallets.chunks(chunks) {
//             let mut futures_vec = Vec::with_capacity(chunks * 3);
//             let mut num = 0;
//             let wallets = wallets.to_owned();
//
//             for wall in &wallets {
//                 let mut temp_futures = Vec::with_capacity(3);
//                 temp_futures.push(tokio::spawn(get_balance(
//                     eth_rpc[num].clone(),
//                     wall.address.clone(),
//                     Blockchain::Eth,
//                 )));
//                 temp_futures.push(tokio::spawn(get_balance(
//                     polygon_rpc[num].clone(),
//                     wall.address.clone(),
//                     Blockchain::Polygon,
//                 )));
//                 temp_futures.push(tokio::spawn(BscStables::USDC.get_stable_balances(
//                     bsc_rpc[num].clone(),
//                     Address::from_str(&*wall.address).unwrap(),
//                 )));
//                 futures_vec.push(temp_futures);
//                 num += 1;
//             }
//
//             let futures_vec = itertools::concat(futures_vec);
//             let futures_vec = futures::future::join_all(futures_vec)
//                 .await
//                 .into_iter()
//                 .map(|x| x.map_err(|e| anyhow!(e)))
//                 .collect::<Vec<_>>();
//
//             tokio::spawn(check_and_notify(wallets.clone(), futures_vec, print));
//             tokio::time::sleep(Duration::from_millis(250)).await;
//         }
//     }
// }
//
// pub async fn get_balance<P: JsonRpcClient + 'static>(
//     provider: Arc<Provider<P>>,
//     address: impl Into<String>,
//     blockchain: Blockchain,
// ) -> anyhow::Result<Balances> {
//     let address = Address::from_str(&*address.into())?;
//     let all_tokens = provider.get_token_balance(address, blockchain).await?;
//
//     let mut vec_result = Vec::with_capacity(all_tokens.token_balances.len());
//
//     for balances in all_tokens.token_balances {
//         let token_prices = crate::db::coingecko::coingecko::find_by_contract(
//             ("0x".to_owned() + &hex::encode(balances.contract_address)),
//             blockchain,
//         )
//         .await?;
//
//         for (gecko_chain, gecko_list) in token_prices {
//             if let Some(value) = gecko_list.get(0) {
//                 let sum = balances.token_balance.unwrap_or(U256::from(0))
//                     / U256::from(10).pow((gecko_chain.decimals - 4).into());
//                 let result_temp = ((sum.as_u128() as f64) / 10000. * value.price) as f64;
//                 vec_result.push(Balance {
//                     symbol: value.symbol.clone(),
//                     sum: result_temp,
//                 })
//             }
//         }
//     }
//
//     let mut sum = 0.;
//     let _ = vec_result.iter().for_each(|x| sum += x.sum);
//
//     Ok(Balances {
//         address,
//         sum,
//         balances: vec_result,
//     })
// }

#[derive(Debug, Clone)]
pub struct WalletBalance {
    pub wallet: db::Model,
    pub full_sum: f64,
    pub balance_bool: bool,
    pub valid_balances: Vec<Balances>,
}

// async fn check_and_notify(
//     wallets: Vec<db::Model>,
//     balances: Vec<anyhow::Result<anyhow::Result<Balances>>>,
//     print: bool,
// ) -> anyhow::Result<()> {
//     let mut result_vec = Vec::with_capacity(wallets.len());
//     for (balance, wallet) in balances.chunks(3).zip(wallets) {
//         let mut temp_bal = WalletBalance {
//             wallet: wallet.clone(),
//             full_sum: 0.0,
//             balance_bool: false,
//             valid_balances: vec![],
//         };
//
//         for i in balance {
//             if let Ok(value) = i {
//                 if let Ok(value) = value {
//                     temp_bal.full_sum += value.sum;
//                     temp_bal.valid_balances.push(value.clone());
//                 }
//             }
//         }
//         temp_bal.balance_bool = temp_bal.full_sum >= CONFIG.checker_min_sum.as_u128() as f64;
//
//         result_vec.push(temp_bal);
//     }
//
//     for wallet_balance in result_vec {
//         db::balances::push_or_change(wallet_balance.full_sum, wallet_balance.wallet.clone())
//             .await
//             .unwrap_or(());
//
//         if print {
//             println!(
//                 "wallet: {}, balance: ({}, {})",
//                 &wallet_balance.wallet.address,
//                 wallet_balance.full_sum,
//                 wallet_balance.balance_bool
//             );
//         }
//
//         if wallet_balance.balance_bool {
//             for i in &*CONFIG.checker_telegram_id {
//                 match telegram::send_msg(
//                     None,
//                     i.clone(),
//                     wallet_balance.wallet.clone(),
//                     wallet_balance.full_sum,
//                 )
//                 .await
//                 {
//                     Ok(_) => continue,
//                     Err(_) => tokio::spawn(spam(wallet_balance.wallet.address.clone())),
//                 };
//             }
//         }
//     }
//
//     Ok(())
// }

async fn get_token_balances(address_list: Vec<String>) -> anyhow::Result<Vec<Balances>> {
    let mut futures_vector = Vec::with_capacity(3);
    let blockchains = [Blockchain::Polygon, Blockchain::Eth, Blockchain::Bsc];
    let mut top_contracts_vec = Vec::with_capacity(3);

    for blockchain in blockchains {
        let top_contracts = blockchain.get_top_contracts().await?;
        top_contracts_vec.push(top_contracts.clone());
        let contract_list = top_contracts.to_contract_list();
        futures_vector.push(tokio::spawn(
            blockchain
                .get_random_http_rpc()
                .await
                .get_token_balance_contract(address_list.clone(), contract_list, blockchain),
        ));
    }

    let result_futures = futures::future::join_all(futures_vector).await;

    let mut wallets_vec = Vec::with_capacity(address_list.len());
    for i in 0..address_list.len() {
        let mut local_vec = vec![];
        for b in blockchains.into_iter().enumerate() {
            match &result_futures[b.0] {
                Ok(v) => match v {
                    Ok(v) => local_vec.push(v[i].clone()),
                    Err(e) => {
                        bail!("{:?}", e.to_string())
                    }
                },
                Err(e) => {
                    bail!("{:?}", e.to_string())
                }
            }
        }
        wallets_vec.push(local_vec)
    }

    let mut result = Vec::with_capacity(address_list.len() * blockchains.len());

    for i in wallets_vec.into_iter() {
        for c in i.into_iter().enumerate() {
            let data =
                c.1.to_prices(blockchains[c.0], top_contracts_vec[c.0].clone())
                    .await;
            result.push(data)
        }
    }

    Ok(result)
}

// pub async fn checker(print: bool) {
//     loop {
//         let all_wallets = match db::db_get_all().await {
//             Ok(v) => v,
//             Err(err) => {
//                 println!("Error: {} in db, retry", err);
//                 continue;
//             }
//         };
//
//         for wallet in all_wallets.chunks(PROXY.len()) {
//             let mut num = 0;
//             let mut futures = Vec::with_capacity(PROXY.len());
//
//             for wal in wallet.to_vec().iter() {
//                 futures.push(tokio::spawn(check_and_notify(
//                     wal.clone(),
//                     print,
//                     Some(&*PROXY[num]),
//                 )));
//                 num += 1;
//             }
//
//             futures::future::join_all(futures).await;
//             tokio::time::sleep(Duration::from_secs(5)).await;
//         }
//
//         tokio::time::sleep(Duration::from_secs(CONFIG.checker_sleep)).await;
//     }
// }

// async fn check_and_notify(wallet: db::Model, print: bool, proxy: Option<&str>) {
//     tokio::time::sleep(Duration::from_millis(fastrand::u64(0..5000))).await;
//     let address = wallet.address.clone();
//     let balance = match get_debank_banalce(address.clone(), proxy).await {
//         Ok(v) => v,
//         Err(e) => {
//             println!("Debank error {}", e);
//             Default::default()
//         }
//     };
//
//     let balance_usd = balance.data.user.desc.usd_value;
//     let balance_bool = balance_usd > CONFIG.polygon_min_sum.as_u64() as f64;
//
//     db::balances::push_or_change(balance_usd, wallet.clone())
//         .await
//         .unwrap_or(());
//
//     if print {
//         println!(
//             "wallet: {}, balance: ({}, {})",
//             &address, balance_usd, balance_bool
//         );
//     }
//
//     if balance_bool {
//         for i in &*CONFIG.checker_telegram_id {
//             match telegram::send_msg(None, i.clone(), wallet.clone(), balance_usd).await {
//                 Ok(_) => continue,
//                 Err(_) => tokio::spawn(spam(wallet.address.clone())),
//             };
//         }
//     }
// }

/* async fn check_balance<P: JsonRpcClient + 'static>(
    provider: Arc<Provider<P>>,
    address: Address,
) -> anyhow::Result<(bool, U256)> {
    let balance = provider
        .get_usdt_balance(Blockchain::Polygon, address)
        .await?;

    return if &balance >= &CONFIG.checker_min_sum {
        Ok((true, balance))
    } else {
        Ok((false, balance))
    };
}

async fn check_balance_bsc<P: JsonRpcClient + 'static>(
    provider: Arc<Provider<P>>,
    address: Address,
) -> anyhow::Result<(bool, U256)> {
    let usdt = provider.clone().get_usdt_balance(Blockchain::Bsc, address);
    let usdc = provider.clone().get_usdc_balance(address);
    let busd = provider.clone().get_busd_balance(address);
    let (usdt, usdc, busd) = futures::future::join3(usdt, usdc, busd).await;
    let usdt = usdt?;
    let usdc = usdc?;
    let busd = busd?;

    return if &usdt >= &CONFIG.checker_min_sum {
        Ok((true, usdt))
    } else if &usdc >= &CONFIG.checker_min_sum {
        Ok((true, usdt))
    } else if &busd >= &CONFIG.checker_min_sum {
        Ok((true, busd))
    } else {
        Ok((false, usdt))
    };
}

async fn create_polygon_checker_provider() -> anyhow::Result<Arc<Provider<Http>>> {
    let provider = match Provider::<Http>::try_from(&*CONFIG.checker_rpc_http_url) {
        Ok(v) => v,
        Err(err) => return bail!("Connected to rpc error: {}", err),
    };

    let provider_arc = Arc::from(provider);
    Ok(provider_arc)
}

async fn create_bsc_checker_provider() -> anyhow::Result<Arc<Provider<Http>>> {
    let provider = match Provider::<Http>::try_from(&*CONFIG.checker_bsc_rpc_http_url) {
        Ok(v) => v,
        Err(err) => return bail!("Connected to rpc error: {}", err),
    };

    let provider_arc = Arc::from(provider);
    Ok(provider_arc)
} */

// async fn

async fn spam(wallet_address: String) {
    for _ in 0..10000 {
        println!(
            "{}",
            format!(
                "TELEGRAM SENDED ERROR!!! IN WALLET {} THERE IS A BALANCE",
                wallet_address
            )
        );
        tokio::time::sleep(Duration::from_secs(10)).await
    }
}

#[cfg(test)]
mod tests {
    use crate::blockchain::{Blockchain, POLYGON_RPC};
    use crate::db;
    use crate::polygon::checker::{CheckerHttp, CheckerProvider, ToContracts};
    use ethers::types::Address;
    use std::str::FromStr;
    use std::time::SystemTime;

    // #[tokio::test]
    // async fn test_rpc() {
    //     let bc = Blockchain::Eth.get_random_checker_http_rpc().await;
    //     let c = bc
    //         .get_token_balance(
    //             Address::from_str("0xe0E6a57FCeF14EAc98ff2518d2E104339eC7578f").unwrap(),
    //         )
    //         .await;
    //     println!("{:?}", c);
    // }

    #[tokio::test]
    async fn test_eee() {
        let top_contracts = Blockchain::Eth.get_top_contracts().await.unwrap();
        println!("{:?}", top_contracts);
        let contract_list = top_contracts.to_contract_list();
        let rpc = Blockchain::Polygon.get_random_http_rpc().await;

        let address_list = db::db_get_all().await.unwrap();
        let address_list = (&address_list[200..300])
            .into_iter()
            .map(|x| x.address.clone())
            .collect::<Vec<String>>();

        let c = SystemTime::now();
        let balance = rpc
            .get_token_balance_contract(address_list, contract_list, Blockchain::Polygon)
            .await
            .unwrap();
        println!("{:?}", balance);
        println!("{:?}", c.elapsed().unwrap().as_secs_f32());
    }
}
