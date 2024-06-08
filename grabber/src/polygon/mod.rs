pub mod checker;
mod checker_abi;
pub mod usdt;

use crate::blockchain::{Blockchain, BlockchainBehivour};
use crate::contracts::EventParams;
use crate::polygon::usdt::USDT_ERC20;
use crate::wallet::generate_wallet;
use crate::{db, Variant, CONFIG};
use anyhow::bail;
use async_trait::async_trait;
use ethers::abi::{Address, Uint};
use ethers::prelude::{
    BlockId, JsonRpcClient, LocalWallet, Middleware, Provider, Signer, SignerMiddleware,
    TransactionReceipt, TransactionRequest, Ws, H256, U256, U64,
};
use ethers::types::BlockNumber;
use lazy_static::lazy_static;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub(crate) const USDT_DECIMALS: u32 = 6;
const GAS_LIMIT: usize = 65000;
const BLOCKCHAIN: Blockchain = Blockchain::Polygon;

lazy_static! {
    pub(crate) static ref USDT_ADDRESS: Address =
        Address::from_str("0xc2132D05D31c914a87C6611C10748AEb04B58e8F").unwrap();
    pub(crate) static ref TRANSFER: H256 =
        H256::from_str("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")
            .unwrap();
    pub(crate) static ref USDT_ABI: ethers::abi::Abi =
        serde_json::from_str((fs::read_to_string("src/polygon/usdt_abi.json").unwrap()).as_str())
            .unwrap();
    pub(crate) static ref USDT_EVENTS: EventParams =
        crate::contracts::find_event(&*USDT_ABI, "Transfer");
    pub static ref BLOCK_ID: BlockId = BlockId::from(BlockNumber::Pending);
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PolygonBlockTransfers {
    pub block: Option<H256>,
    pub transfers: Vec<PolygonTransfer>,
    pub transfer_native: Vec<MaticTransfer>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PolygonTransfer {
    pub transaction_hash: H256,
    pub transfers: Vec<PolygonUsdtTransfer>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PolygonUsdtTransfer {
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub balance_from: U256,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MaticTransfer {
    pub transaction_hash: H256,
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub balance_from: U256,
}

#[async_trait]
trait Behivour: Sync + Send + Debug {
    async fn get_format_gas_price(self: Arc<Self>) -> anyhow::Result<U256>;
    async fn send_usdt(
        self: Arc<Self>,
        address: Address,
        wallet: Option<LocalWallet>,
        sum: Option<u128>,
        gas_price: U256,
        nonce: Option<U256>,
        block: Option<U64>,
    ) -> anyhow::Result<TransactionReceipt>;
    async fn send_matic(
        self: Arc<Self>,
        address: Address,
        wallet: Option<LocalWallet>,
        gas_price: U256,
        nonce: Option<U256>,
        value: Option<U256>,
    ) -> anyhow::Result<TransactionReceipt>;
    async fn send_matic_and_usdt(
        self: Arc<Self>,
        address: Address,
        wallet: Option<LocalWallet>,
        nonce: Arc<AtomicUsize>,
        usdt_sum: Option<u128>,
    ) -> anyhow::Result<(U256, TransactionReceipt)>;
}

#[async_trait]
#[allow(unreachable_code)]
impl<P: JsonRpcClient + 'static> Behivour for Provider<P> {
    async fn get_format_gas_price(self: Arc<Self>) -> anyhow::Result<U256> {
        let mut gas_price = self.get_gas_price().await?;
        gas_price /= U256::from(100);
        gas_price *= U256::from(150);
        Ok(gas_price)
    }

    // передавать сумму в обычном количестве коинов, оно само переведет в блокчейн систему
    #[allow(unused_variables)]
    async fn send_usdt(
        self: Arc<Self>,
        address: Address,
        wallet: Option<LocalWallet>,
        sum: Option<u128>,
        gas_price: U256,
        nonce: Option<U256>,
        block: Option<U64>,
    ) -> anyhow::Result<TransactionReceipt> {
        let wallet = match wallet {
            None => (&CONFIG.polygon_default_wallet).clone(),
            Some(v) => v,
        };

        println!(
            "send usdt, to: {:?}, from: {:?} nonce: {:?}",
            &address,
            &wallet.address(),
            &nonce
        );

        let wallet = SignerMiddleware::new_with_provider_chain(self.clone(), wallet).await?;
        let contract = USDT_ERC20::new(*USDT_ADDRESS, Arc::from(wallet.clone()));

        let sum = match sum {
            None => CONFIG.polygon_transfer_sum,
            Some(v) => U256::from(v * 10_i32.pow(USDT_DECIMALS) as u128),
        };

        let gas_price = ((gas_price / U256::from(100)) * U256::from(80)) - (sum + U256::from(300));

        let contract_call = contract.transfer(address, Uint::from(sum));
        let tx = contract_call.tx.as_eip1559_ref().unwrap();
        let mut tx = tx
            .clone()
            .nonce(nonce.unwrap_or(0.into()))
            .gas(GAS_LIMIT)
            .max_fee_per_gas(gas_price)
            .max_priority_fee_per_gas(gas_price);

        let tx = match wallet.send_transaction(tx.clone(), None).await?.await? {
            None => return bail!("Error option when send USDT Matic, most likely a bug"),
            Some(v) => v,
        };

        Ok(tx)
    }

    async fn send_matic(
        self: Arc<Self>,
        address: Address,
        wallet: Option<LocalWallet>,
        gas_price: U256,
        nonce: Option<U256>,
        value: Option<U256>,
    ) -> anyhow::Result<TransactionReceipt> {
        let wallet = match wallet {
            None => (&CONFIG.polygon_default_wallet).clone(),
            Some(v) => v,
        };

        println!(
            "send matic, to: {:?}, from: {:?} nonce: {:?}",
            &address,
            &wallet.address(),
            &nonce
        );

        let wallet = SignerMiddleware::new_with_provider_chain(self.clone(), wallet).await?;

        let value = match value {
            None => U256::from(GAS_LIMIT) * gas_price,
            Some(v) => v,
        };
        let mut tx = TransactionRequest::new()
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

    #[allow(unused_variables)]
    async fn send_matic_and_usdt(
        self: Arc<Self>,
        address: Address,
        wallet: Option<LocalWallet>,
        nonce: Arc<AtomicUsize>,
        usdt_sum: Option<u128>,
    ) -> anyhow::Result<(U256, TransactionReceipt)> {
        let nonce_matic = nonce.load(Ordering::Relaxed);
        nonce.fetch_add(1, Ordering::Relaxed);
        let nonce_usdt = nonce.load(Ordering::Relaxed);
        nonce.fetch_add(1, Ordering::Relaxed);

        let gas_price = self.clone().get_format_gas_price().await?;

        let matic = self.clone().send_matic(
            address,
            wallet.clone(),
            gas_price,
            Some(nonce_matic.into()),
            None,
        );
        let usdt = self.clone().send_usdt(
            address,
            wallet.clone(),
            None,
            gas_price,
            Some(nonce_usdt.into()),
            None,
        );

        let (matic, usdt) = futures::future::join(matic, usdt).await;
        let _matic = matic?;
        let usdt = usdt?;

        Ok((gas_price, usdt))
    }
}

pub(crate) async fn main_func(
    block_hash: H256,
    nonce: Arc<AtomicUsize>,
    limit: Arc<AtomicUsize>,
    variant: Variant,
    native: bool,
) -> anyhow::Result<()> {
    let mut provider = BLOCKCHAIN.get_random_http_rpc().await;

    let wallet_balance = provider.get_balance(CONFIG.polygon_default_wallet_address, None).await?;
    let need_balance = ethers::utils::parse_units("0.1", "ether").unwrap().into();
    if wallet_balance <= need_balance {
        eprintln!("polygon insufficient balance. your balance: {:?}, but need: {:?}", wallet_balance, need_balance);
        bail!("polygon insufficient balance. your balance: {:?}, but need: {:?}", wallet_balance, need_balance)
    }

    let block_transfers = loop {
        match polygon_get_block_info(block_hash, provider.clone()).await {
            Ok(v) => {
                break v;
            }
            Err(e) => {
                println!(
                    "polygon_get_block_info error: {:?}, retying; node {:?}",
                    e,
                    provider.url()
                );
                provider = BLOCKCHAIN.get_random_http_rpc().await;
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
            Blockchain::Polygon,
            transaction_number,
            &block_hash
        );
    }

    let mut future_tasks = Vec::with_capacity(wallet_send.len());

    for i in &wallet_send {
        if limit.load(Ordering::Relaxed) >= CONFIG.polygon_max_transfers.load(Ordering::Relaxed) {
            println!("limit");
            return Ok(());
        }

        match variant {
            Variant::AaBb => {
                future_tasks.push(main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.from,
                    i.from,
                ));
                future_tasks.push(main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.to,
                    i.to,
                ));
            }
            Variant::AbBa => {
                future_tasks.push(main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.to,
                    i.from,
                ));
                future_tasks.push(main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.from,
                    i.to,
                ));
            }
            Variant::All => {
                future_tasks.push(main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.from,
                    i.from,
                ));
                future_tasks.push(main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.to,
                    i.to,
                ));
                future_tasks.push(main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.to,
                    i.from,
                ));
                future_tasks.push(main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    BLOCKCHAIN.get_random_http_rpc().await,
                    i.from,
                    i.to,
                ));
            }
            Variant::Default => {
                future_tasks.push(main_two_func(
                    nonce.clone(),
                    limit.clone(),
                    provider.clone(),
                    i.to,
                    i.from,
                ));
            }
        }
    }

    let _ = futures::future::join_all(future_tasks).await;

    if native {
        let mut future_tasks = Vec::with_capacity(block_transfers.transfer_native.len());

        for polygon_transfer in block_transfers.transfer_native {
            match variant {
                Variant::AaBb => {
                    future_tasks.push(matic_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        polygon_transfer.from,
                        polygon_transfer.from,
                    ));
                    future_tasks.push(matic_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        polygon_transfer.to,
                        polygon_transfer.to,
                    ));
                }
                Variant::AbBa => {
                    future_tasks.push(matic_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        polygon_transfer.to,
                        polygon_transfer.from,
                    ));
                    future_tasks.push(matic_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        polygon_transfer.from,
                        polygon_transfer.to,
                    ));
                }
                Variant::All => {
                    future_tasks.push(matic_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        polygon_transfer.from,
                        polygon_transfer.from,
                    ));
                    future_tasks.push(matic_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        polygon_transfer.to,
                        polygon_transfer.to,
                    ));
                    future_tasks.push(matic_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        polygon_transfer.to,
                        polygon_transfer.from,
                    ));
                    future_tasks.push(matic_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        polygon_transfer.from,
                        polygon_transfer.to,
                    ));
                }
                Variant::Default => {
                    future_tasks.push(matic_main_two_func(
                        nonce.clone(),
                        limit.clone(),
                        BLOCKCHAIN.get_random_http_rpc().await,
                        polygon_transfer.to,
                        polygon_transfer.from,
                    ));
                }
            }
        }

        let _ = futures::future::join_all(future_tasks).await;
    }

    Ok(())
}

async fn main_two_func<P: JsonRpcClient + 'static>(
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

    let (mut gas_price, receipt) = match provider
        .clone()
        .send_matic_and_usdt(wallet.address, None, nonce.clone(), None)
        .await
    {
        Ok((v, e)) => (v, e),
        Err(e) => {
            println!("Error in main wallet: {:?}", e);
            return Ok(());
        }
    };

    tokio::time::sleep(Duration::from_secs(CONFIG.polygon_interval_send)).await;

    loop {
        match provider
            .clone()
            .send_usdt(
                send_to,
                Some(wallet.private_key.clone()),
                None,
                gas_price,
                None,
                receipt.block_number,
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
                gas_price = provider.clone().get_format_gas_price().await?;
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }
        };
    }

    db::push_to_db(
        send_to,
        &wallet.private_key,
        &wallet.priv_key_string,
        Blockchain::Polygon,
    )
    .await?;

    limit.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

async fn matic_main_two_func<P: JsonRpcClient + 'static>(
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
        .unwrap_or(100000000000_u128.into());
    let value = CONFIG.polygon_transfer_sum_native + U256::from(21000) * gas_price;
    let _ = match provider
        .clone()
        .send_matic(
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

    tokio::time::sleep(Duration::from_secs(CONFIG.polygon_interval_send)).await;

    loop {
        match provider
            .clone()
            .send_matic(
                send_to,
                Some(wallet.private_key.clone()),
                gas_price,
                None,
                Some(CONFIG.polygon_transfer_sum_native),
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
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }

    limit.fetch_add(1, Ordering::Relaxed);

    Ok(())
}

async fn polygon_get_block_info<P: JsonRpcClient + 'static>(
    block_hash: H256,
    provider: Arc<Provider<P>>,
) -> anyhow::Result<PolygonBlockTransfers> {
    let transactions =
        again::retry(|| Blockchain::Polygon.get_receipts_by_block(provider.clone(), block_hash))
            .await?;

    Ok(Blockchain::polygon_get_transfer(transactions, block_hash, provider).await)
}

#[cfg(test)]
mod tests {
    use crate::polygon::{Behivour, USDT_EVENTS};
    use crate::CONFIG;

    use ethers::prelude::Address;

    use std::str::FromStr;

    // #[tokio::test()]
    // async fn test_matic_usdt() {
    //     let provider = create_provider().await.unwrap();
    //     let gas_price = provider.clone().get_format_gas_price().await.unwrap();
    //     let nonce = provider
    //         .clone()
    //         .get_nonce(CONFIG.default_wallet_address)
    //         .await
    //         .unwrap();
    //     let matic = provider.clone().send_matic(
    //         Address::from_str("0x89d3acea16450373a22bcdf1cd3bccc4c8b569ac").unwrap(),
    //         None,
    //         gas_price,
    //         nonce,
    //     );
    //     let usdt = provider.send_usdt(
    //         Address::from_str("0x89d3acea16450373a22bcdf1cd3bccc4c8b569ac").unwrap(),
    //         None,
    //         None,
    //         gas_price,
    //         nonce,
    //     );
    //     let (matic, usdt) = futures::future::join(matic, usdt).await;
    //     matic.unwrap();
    //     usdt.unwrap();
    // }
    //
    // #[tokio::test()]
    // async fn test_send_matic() {
    //     let provider = create_provider().await.unwrap();
    //     let gas_price = provider.clone().get_format_gas_price().await.unwrap();
    //     let nonce = provider
    //         .clone()
    //         .get_nonce(CONFIG.polygon_default_wallet_address)
    //         .await
    //         .unwrap();
    //     provider
    //         .send_matic(
    //             Address::from_str("0x89d3acea16450373a22bcdf1cd3bccc4c8b569ac").unwrap(),
    //             None,
    //             gas_price,
    //             Some(16.into()),
    //         )
    //         .await
    //         .unwrap();
    // }
    //
    // #[tokio::test()]
    // async fn test_send_usdt() {
    //     let provider = create_provider().await.unwrap();
    //     let gas_price = provider.clone().get_format_gas_price().await.unwrap();
    //     let nonce = provider
    //         .clone()
    //         .get_nonce(CONFIG.default_wallet_address)
    //         .await
    //         .unwrap();
    //     provider
    //         .send_usdt(
    //             Address::from_str("0x89d3acea16450373a22bcdf1cd3bccc4c8b569ac").unwrap(),
    //             None,
    //             None,
    //             gas_price,
    //             nonce,
    //         )
    //         .await
    //         .unwrap();
    // }

    #[tokio::test()]
    async fn test_config() {
        println!("{:?}", *CONFIG);
    }
}
