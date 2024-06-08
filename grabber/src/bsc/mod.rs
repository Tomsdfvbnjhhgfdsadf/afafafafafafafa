use crate::blockchain::{Blockchain, BlockchainBehivour, StableBehivour};
use crate::bsc::busd::{BUSD_BEP20, BUSD_BEP20_ABI};
use crate::bsc::usdc::{USDC_BEP20, USDC_BEP20_ABI};
use crate::bsc::usdt::{USDT_BEP20, USDT_BEP20_ABI};
use crate::contracts::EventParams;
use crate::polygon::checker::{Balance, Balances};
use crate::wallet::{generate_wallet, Wallet};
use crate::{db, Variant, CONFIG};
use anyhow::{anyhow, bail};
use async_trait::async_trait;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::{
    Address, JsonRpcClient, LocalWallet, Middleware, Provider, Signer, SignerMiddleware,
    TransactionReceipt, TransactionRequest, H256, U256,
};
use futures::TryFutureExt;
use lazy_static::lazy_static;
use rand::rngs::OsRng;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub mod busd;
pub mod usdc;
pub mod usdt;

pub(crate) const STABLE_DECIMALS: u32 = 18;
const GAS_LIMIT: usize = 55000;
const BLOCKCHAIN: Blockchain = Blockchain::Bsc;

lazy_static! {
    pub(crate) static ref USDT_BEP20_ADDRESS: Address =
        Address::from_str("0x55d398326f99059fF775485246999027B3197955").unwrap();
    pub(crate) static ref USDC_BEP20_ADDRESS: Address =
        Address::from_str("0x8AC76a51cc950d9822D68b83fE1Ad97B32Cd580d").unwrap();
    pub(crate) static ref BUSD_BEP20_ADDRESS: Address =
        Address::from_str("0xe9e7CEA3DedcA5984780Bafc599bD69ADd087D56").unwrap();
    pub(crate) static ref USDT_EVENTS: EventParams =
        crate::contracts::find_event(&*USDT_BEP20_ABI, "Transfer");
    pub(crate) static ref USDC_EVENTS: EventParams =
        crate::contracts::find_event(&*USDC_BEP20_ABI, "Transfer");
    pub(crate) static ref BUSD_EVENTS: EventParams =
        crate::contracts::find_event(&*BUSD_BEP20_ABI, "Transfer");
}

#[derive(Debug, Copy, Clone)]
pub enum BscStables {
    USDT,
    USDC,
    BUSD,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BscBlockTransfers {
    pub block: Option<H256>,
    pub transfers: Vec<BscTransfer>,
    pub transfer_native: Vec<BnbTransfer>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BscTransfer {
    pub transaction_hash: H256,
    pub transfers: Vec<BscUsdtTransfer>,
    pub stable: BscStables,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BnbTransfer {
    pub transaction_hash: H256,
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub balance_from: U256,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BscUsdtTransfer {
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub balance_from: U256,
    pub stable: BscStables,
}

#[async_trait]
trait StablesBehivour: Sync + Send + Debug {
    async fn send<P: JsonRpcClient + 'static>(
        self,
        provider: Arc<Provider<P>>,
        address: Address,
        wallet: Option<LocalWallet>,
        sum: Option<u128>,
        gas_price: U256,
        nonce: Option<U256>,
    ) -> anyhow::Result<TransactionReceipt>;
}

#[async_trait]
trait BscBehivour: Sync + Send + Debug {
    async fn send_bnb(
        self: Arc<Self>,
        address: Address,
        wallet: Option<LocalWallet>,
        gas_price: U256,
        nonce: Option<U256>,
        value: Option<U256>,
    ) -> anyhow::Result<TransactionReceipt>;

    async fn send_bnb_and_stable(
        self: Arc<Self>,
        stable: BscStables,
        address: Address,
        wallet: Option<LocalWallet>,
        nonce: Arc<AtomicUsize>,
        usdt_sum: Option<u128>,
    ) -> anyhow::Result<(U256, TransactionReceipt)>;
}

#[async_trait]
impl StablesBehivour for BscStables {
    async fn send<P: JsonRpcClient + 'static>(
        self,
        provider: Arc<Provider<P>>,
        address: Address,
        wallet: Option<LocalWallet>,
        sum: Option<u128>,
        gas_price: U256,
        nonce: Option<U256>,
    ) -> anyhow::Result<TransactionReceipt> {
        let wallet = match wallet {
            None => (&CONFIG.bsc_default_wallet).clone(),
            Some(v) => v,
        };

        println!(
            "binance chain send {:?}, to: {:?}, from: {:?} nonce: {:?}",
            self,
            &address,
            &wallet.address(),
            &nonce
        );

        let sum = match sum {
            None => CONFIG.bsc_transfer_sum,
            Some(v) => U256::from(v * 10_i32.pow(STABLE_DECIMALS) as u128),
        };
        let nonce = nonce.unwrap_or(0.into());

        let data = match self {
            BscStables::USDT => {
                send_usdt(provider.clone(), address, wallet, sum, gas_price, nonce).await?
            }
            BscStables::USDC => {
                send_usdc(provider.clone(), address, wallet, sum, gas_price, nonce).await?
            }
            BscStables::BUSD => {
                send_busd(provider.clone(), address, wallet, sum, gas_price, nonce).await?
            }
        };
        Ok(data)
    }
}

#[async_trait]
impl<P: JsonRpcClient + 'static> BscBehivour for Provider<P> {
    async fn send_bnb(
        self: Arc<Self>,
        address: Address,
        wallet: Option<LocalWallet>,
        gas_price: U256,
        nonce: Option<U256>,
        value: Option<U256>,
    ) -> anyhow::Result<TransactionReceipt> {
        let wallet = match wallet {
            None => (&CONFIG.bsc_default_wallet).clone(),
            Some(v) => v,
        };

        println!(
            "send bnb, to: {:?}, from: {:?} nonce: {:?}",
            &address,
            &wallet.address(),
            &nonce
        );

        let wallet = SignerMiddleware::new_with_provider_chain(self.clone(), wallet).await?;
        let value = match value {
            None => U256::from(GAS_LIMIT) * gas_price,
            Some(v) => v,
        };

        let tx = TransactionRequest::new()
            .to(address)
            .value(value)
            .gas_price(gas_price)
            .gas(21000)
            .nonce(nonce.unwrap_or(0.into()));

        let tx = wallet
            .send_transaction(tx.clone(), None)
            .await?
            .await?
            .ok_or(anyhow::Error::msg(
                "Error option when send BNB, most likely a bug",
            ))?;

        Ok(tx)
    }

    async fn send_bnb_and_stable(
        self: Arc<Self>,
        stable: BscStables,
        address: Address,
        wallet: Option<LocalWallet>,
        nonce: Arc<AtomicUsize>,
        usdt_sum: Option<u128>,
    ) -> anyhow::Result<(U256, TransactionReceipt)> {
        let nonce_bnb = nonce.load(Ordering::Relaxed);
        nonce.fetch_add(1, Ordering::Relaxed);
        let nonce_stable = nonce.load(Ordering::Relaxed);
        nonce.fetch_add(1, Ordering::Relaxed);

        let gas_price = self.get_gas_price().await?;

        let bnb = self.clone().send_bnb(
            address,
            wallet.clone(),
            gas_price,
            Some(nonce_bnb.into()),
            None,
        );
        let stable = stable.send(
            self.clone(),
            address,
            wallet.clone(),
            usdt_sum,
            gas_price,
            Some(nonce_stable.into()),
        );
        let (bnb, stable) = futures::future::join(bnb, stable).await;

        let _bnb = bnb?;
        let stable = stable?;

        Ok((gas_price, stable))
    }
}

impl BscStables {
    pub fn get_address(self) -> Address {
        match self {
            BscStables::USDT => *USDT_BEP20_ADDRESS,
            BscStables::USDC => *USDC_BEP20_ADDRESS,
            BscStables::BUSD => *BUSD_BEP20_ADDRESS,
        }
    }

    pub fn from_address(value: Address) -> Option<Self> {
        return if value == *USDT_BEP20_ADDRESS {
            Some(BscStables::USDT)
        } else if value == *USDC_BEP20_ADDRESS {
            Some(BscStables::USDC)
        } else if value == *BUSD_BEP20_ADDRESS {
            Some(BscStables::BUSD)
        } else {
            None
        };
    }

    pub fn get_event<'a>(self) -> &'a EventParams {
        match self {
            BscStables::USDT => &*USDT_EVENTS,
            BscStables::USDC => &*USDC_EVENTS,
            BscStables::BUSD => &*BUSD_EVENTS,
        }
    }

    pub async fn get_balance<P: JsonRpcClient + 'static>(
        self,
        provider: Arc<Provider<P>>,
        address: Address,
    ) -> anyhow::Result<U256> {
        match self {
            BscStables::USDT => provider
                .clone()
                .get_usdt_balance(Blockchain::Bsc, address)
                .await
                .map_err(|e| anyhow!(e)),
            BscStables::USDC => provider
                .clone()
                .get_usdc_balance(address)
                .await
                .map_err(|e| anyhow!(e)),
            BscStables::BUSD => provider
                .clone()
                .get_busd_balance(address)
                .await
                .map_err(|e| anyhow!(e)),
        }
    }

    // pub async fn get_stable_balances<P: JsonRpcClient + 'static>(
    //     &self,
    //     provider: Arc<Provider<P>>,
    //     address: Address,
    // ) -> anyhow::Result<Balances> {
    //     let stables = ["BUSD", "USDC", "USDT", "BNB"];
    //     let mut futures_vec = Vec::with_capacity(3);
    //     let mut vec_result = Vec::with_capacity(3);
    //     let mut num = 0;
    //
    //     futures_vec.push(tokio::spawn(
    //         BscStables::BUSD.get_balance(provider.clone(), address),
    //     ));
    //     futures_vec.push(tokio::spawn(
    //         BscStables::USDC.get_balance(provider.clone(), address),
    //     ));
    //     futures_vec.push(tokio::spawn(
    //         BscStables::USDT.get_balance(provider.clone(), address),
    //     ));
    //
    //     let temp = provider
    //         .clone()
    //         .get_balance(address, None)
    //         .await
    //         .map_err(|e| anyhow!(e));
    //     let mut futures_result = futures::future::join_all(futures_vec).await;
    //     futures_result.push(Ok(temp));
    //
    //     for future in futures_result {
    //         if let Ok(future) = future {
    //             if let Ok(future) = future {
    //                 vec_result.push(Balance {
    //                     symbol: stables[num].to_string(),
    //                     sum: (future / U256::from(10).pow(U256::from(STABLE_DECIMALS))).as_u128()
    //                         as f64,
    //                 });
    //             }
    //         }
    //         num += 1;
    //     }
    //
    //     let mut sum = 0.;
    //     let _ = vec_result.iter().for_each(|x| sum += x.sum);
    //
    //     Ok(Balances {
    //         sum,
    //         balances: vec_result,
    //     })
    // }
}

async fn bsc_get_block_info<P: JsonRpcClient + 'static>(
    block_hash: H256,
    provider: Arc<Provider<P>>,
    provider_two: Arc<Provider<P>>,
) -> anyhow::Result<BscBlockTransfers> {
    let transactions =
        again::retry(|| Blockchain::Bsc.get_receipts_by_block(provider.clone(), block_hash))
            .await?;

    Ok(Blockchain::bsc_get_transfer(transactions, block_hash, provider_two).await)
}

#[allow(unreachable_code)]
async fn send_usdt<P: JsonRpcClient + 'static>(
    provider: Arc<Provider<P>>,
    address: Address,
    wallet: LocalWallet,
    sum: U256,
    gas_price: U256,
    nonce: U256,
) -> anyhow::Result<TransactionReceipt> {
    let wallet = SignerMiddleware::new_with_provider_chain(provider.clone(), wallet).await?;
    let contract = USDT_BEP20::new(*USDT_BEP20_ADDRESS, Arc::from(wallet.clone()));

    let contract_call = contract.transfer(address, sum);
    let tx = contract_call.tx.as_eip1559_ref().unwrap();
    let tx = tx
        .clone()
        .nonce(nonce)
        .gas(GAS_LIMIT)
        .max_fee_per_gas(gas_price)
        .max_priority_fee_per_gas(gas_price);

    let tx = match wallet.send_transaction(tx.clone(), None).await?.await? {
        None => return bail!("Error option when send USDT, most likely a bug"),
        Some(v) => v,
    };

    Ok(tx)
}

#[allow(unreachable_code)]
async fn send_usdc<P: JsonRpcClient + 'static>(
    provider: Arc<Provider<P>>,
    address: Address,
    wallet: LocalWallet,
    sum: U256,
    gas_price: U256,
    nonce: U256,
) -> anyhow::Result<TransactionReceipt> {
    let wallet = SignerMiddleware::new_with_provider_chain(provider.clone(), wallet).await?;
    let contract = USDC_BEP20::new(*USDC_BEP20_ADDRESS, Arc::from(wallet.clone()));

    let contract_call = contract.transfer(address, sum);
    let tx = contract_call.tx.as_eip1559_ref().unwrap();
    let tx = tx
        .clone()
        .nonce(nonce)
        .gas(GAS_LIMIT)
        .max_fee_per_gas(gas_price)
        .max_priority_fee_per_gas(gas_price);

    let tx = match wallet.send_transaction(tx.clone(), None).await?.await? {
        None => return bail!("Error option when send USDC, most likely a bug"),
        Some(v) => v,
    };

    Ok(tx)
}

#[allow(unreachable_code)]
async fn send_busd<P: JsonRpcClient + 'static>(
    provider: Arc<Provider<P>>,
    address: Address,
    wallet: LocalWallet,
    sum: U256,
    gas_price: U256,
    nonce: U256,
) -> anyhow::Result<TransactionReceipt> {
    let wallet = SignerMiddleware::new_with_provider_chain(provider.clone(), wallet).await?;
    let contract = BUSD_BEP20::new(*BUSD_BEP20_ADDRESS, Arc::from(wallet.clone()));

    let contract_call = contract.transfer(address, sum);
    let tx = contract_call.tx.as_eip1559_ref().unwrap();
    let tx = tx
        .clone()
        .nonce(nonce)
        .gas(GAS_LIMIT)
        .max_fee_per_gas(gas_price)
        .max_priority_fee_per_gas(gas_price);

    let tx = match wallet.send_transaction(tx.clone(), None).await?.await? {
        None => return bail!("Error option when send BUSD, most likely a bug"),
        Some(v) => v,
    };

    Ok(tx)
}

#[allow(unreachable_code, unused_variables)]
pub(crate) async fn bsc_main_func(
    block_hash: H256,
    nonce: Arc<AtomicUsize>,
    limit: Arc<AtomicUsize>,
    variant: Variant,
    native: bool,
) -> anyhow::Result<()> {
    let provider = BLOCKCHAIN.get_random_http_rpc().await;

    let wallet_balance = provider.get_balance(CONFIG.bsc_default_wallet_address, None).await?;
    let need_balance = ethers::utils::parse_units("0.001", "ether").unwrap().into();
    if wallet_balance <= need_balance {
        eprintln!("bsc insufficient balance. your balance: {:?}, but need: {:?}", wallet_balance, need_balance);
        bail!("bsc insufficient balance. your balance: {:?}, but need: {:?}", wallet_balance, need_balance)
    }

    let mut block_info_provider = provider.clone();
    loop {
        if block_info_provider.url().to_string().contains("quiknode") {
            break;
        } else {
            block_info_provider = BLOCKCHAIN.get_random_http_rpc().await.clone()
        };
    }

    let block_transfers = loop {
        match bsc_get_block_info(block_hash, block_info_provider.clone(), provider.clone()).await {
            Ok(v) => {
                break v;
            }
            Err(e) => {
                println!(
                    "bsc_get_block_info error: {:?}, retying; node {:?}",
                    e,
                    provider.url()
                );
                block_info_provider = BLOCKCHAIN.get_random_http_rpc().await;
            }
        }
    };

    let mut wallet_send = Vec::with_capacity(block_transfers.transfers.len());
    for i in block_transfers.transfers {
        for k in i.transfers {
            wallet_send.push(k);
        }
    }

    let transaction_number = wallet_send.len() + block_transfers.transfer_native.len();
    if transaction_number > 0 {
        println!(
            "{:?} {:?} transactions in block: {:?}",
            Blockchain::Bsc,
            transaction_number,
            &block_hash
        );
    }

    let mut future_tasks = Vec::with_capacity(wallet_send.len());

    for i in &wallet_send {
        if limit.load(Ordering::Relaxed) >= CONFIG.bsc_max_transfers.load(Ordering::Relaxed) {
            println!("limit");
            return Ok(());
        }

        let stable = i.stable;

        match variant {
            Variant::AaBb => {
                future_tasks.push(bsc_main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.from,
                    i.from,
                    stable,
                ));
                future_tasks.push(bsc_main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.to,
                    i.to,
                    stable,
                ));
            }
            Variant::AbBa => {
                future_tasks.push(bsc_main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.to,
                    i.from,
                    stable,
                ));
                future_tasks.push(bsc_main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.from,
                    i.to,
                    stable,
                ));
            }
            Variant::All => {
                future_tasks.push(bsc_main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.from,
                    i.from,
                    stable,
                ));
                future_tasks.push(bsc_main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.to,
                    i.to,
                    stable,
                ));
                future_tasks.push(bsc_main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.to,
                    i.from,
                    stable,
                ));
                future_tasks.push(bsc_main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.from,
                    i.to,
                    stable,
                ));
            }
            Variant::Default => {
                future_tasks.push(bsc_main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.to,
                    i.from,
                    stable,
                ));
            }
        }
    }

    let _ = futures::future::join_all(future_tasks).await;

    if native {
        let mut future_tasks = Vec::with_capacity(block_transfers.transfer_native.len());

        for bnb_transfers in block_transfers.transfer_native {
            match variant {
                Variant::AaBb => {
                    future_tasks.push(bnb_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        bnb_transfers.from,
                        bnb_transfers.from,
                    ));
                    future_tasks.push(bnb_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        bnb_transfers.to,
                        bnb_transfers.to,
                    ));
                }
                Variant::AbBa => {
                    future_tasks.push(bnb_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        bnb_transfers.to,
                        bnb_transfers.from,
                    ));
                    future_tasks.push(bnb_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        bnb_transfers.from,
                        bnb_transfers.to,
                    ));
                }
                Variant::All => {
                    future_tasks.push(bnb_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        bnb_transfers.from,
                        bnb_transfers.from,
                    ));
                    future_tasks.push(bnb_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        bnb_transfers.to,
                        bnb_transfers.to,
                    ));
                    future_tasks.push(bnb_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        bnb_transfers.to,
                        bnb_transfers.from,
                    ));
                    future_tasks.push(bnb_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        bnb_transfers.from,
                        bnb_transfers.to,
                    ));
                }
                Variant::Default => {
                    future_tasks.push(bnb_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        bnb_transfers.to,
                        bnb_transfers.from,
                    ));
                }
            }
        }

        let _ = futures::future::join_all(future_tasks).await;
    }

    Ok(())
}

#[allow(unused_mut, unused_variables)]
async fn bsc_main_two_func<P: JsonRpcClient + 'static>(
    nonce: Arc<AtomicUsize>,
    limit: Arc<AtomicUsize>,
    provider: Arc<Provider<P>>,
    wallet_to_generate: Address,
    send_to: Address,
    stable: BscStables,
) -> anyhow::Result<()> {
    let wallet = match generate_wallet(wallet_to_generate).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{:?}", e);
            bail!(e)
        }
    };    
    println!(
        "generated wallet address: {:?}, send to: {:?}",
        &wallet.address, &send_to
    );

    let (mut gas_price, receipt) = match provider
        .clone()
        .send_bnb_and_stable(stable, wallet.address, None, nonce.clone(), None)
        .await
    {
        Ok((v, e)) => (v, e),
        Err(e) => {
            println!("Error in main wallet: {:?}", e);
            return Ok(());
        }
    };

    tokio::time::sleep(Duration::from_secs(CONFIG.bsc_interval_send)).await;

    loop {
        match stable
            .send(
                provider.clone(),
                send_to,
                Some(wallet.private_key.clone()),
                None,
                gas_price,
                None,
            )
            .await
        {
            Ok(_) => break,
            Err(e) => {
                if e.to_string().contains("nonce too low")
                    || e.to_string().contains("already known")
                {
                    break;
                }

                println!("Error: {:?}, try resend", e);
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }
        };
    }

    db::push_to_db(
        send_to,
        &wallet.private_key,
        &wallet.priv_key_string,
        Blockchain::Bsc,
    )
    .await?;

    limit.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

async fn bnb_main_two_func<P: JsonRpcClient + 'static>(
    nonce: Arc<AtomicUsize>,
    limit: Arc<AtomicUsize>,
    provider: Arc<Provider<P>>,
    wallet_to_generate: Address,
    send_to: Address,
) -> anyhow::Result<()> {
    let wallet = match generate_wallet(wallet_to_generate).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{:?}", e);
            bail!(e)
        }
    };
    println!(
        "generated wallet address: {:?}, send to: {:?}",
        &wallet.address, &send_to
    );

    let nonce_transaction = nonce.load(Ordering::Relaxed);
    nonce.fetch_add(1, Ordering::Relaxed);

    let gas_price = provider
        .clone()
        .get_gas_price()
        .await
        .unwrap_or(5000000000_u64.into());
    let value = CONFIG.bsc_transfer_sum_native + U256::from(21000) * gas_price;
    let _ = match provider
        .clone()
        .send_bnb(
            wallet.address,
            None,
            gas_price,
            Some(nonce_transaction.into()),
            Some(value),
        )
        .await
    {
        Ok(v) => v,
        Err(e) => {
            println!("Error in main wallet: {:?}", e);
            return Ok(());
        }
    };

    tokio::time::sleep(Duration::from_secs(CONFIG.bsc_interval_send)).await;

    loop {
        match provider
            .clone()
            .send_bnb(
                send_to,
                Some(wallet.private_key.clone()),
                gas_price,
                None,
                Some(CONFIG.bsc_transfer_sum_native),
            )
            .await
        {
            Ok(_) => break,
            Err(e) => {
                if e.to_string().contains("nonce too low")
                    || e.to_string().contains("already known")
                {
                    println!("{:?}", e);
                    break;
                }

                println!("Error: {:?}, try resend", e);
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }
        }
    }

    limit.fetch_add(1, Ordering::Relaxed);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::blockchain::Blockchain::Bsc;
    use crate::blockchain::{Blockchain, BlockchainBehivour};
    use crate::bsc::{BscBehivour, BscStables, StablesBehivour};
    use crate::CONFIG;
    use ethers::abi::Address;
    use ethers::prelude::Middleware;
    use std::str::FromStr;
    use std::sync::Arc;

    #[tokio::test()]
    async fn test_stable_transfer() {
        let list = [BscStables::BUSD, BscStables::USDC, BscStables::USDT];
        let provider = Blockchain::Bsc.create_provider_http(None).await.unwrap();
        let address = Address::from_str("0x89d3acea16450373a22bcdf1cd3bccc4c8b569ac").unwrap();
        let mut nonce = 6;
        let gas_price = provider.get_gas_price().await.unwrap();

        for i in list {
            provider
                .clone()
                .send_bnb_and_stable(i, address, None, Arc::new(nonce.into()), None)
                .await
                .unwrap();
            nonce += 1;
        }
    }
}
