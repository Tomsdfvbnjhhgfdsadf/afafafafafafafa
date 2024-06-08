pub(crate) mod usdt;

use crate::blockchain::{Blockchain, BlockchainBehivour};
use crate::contracts::EventParams;
use crate::eth::usdt::USDT_ETH20Token;
use crate::wallet::generate_wallet;
use crate::{db, Variant, CONFIG};
use anyhow::bail;
use async_trait::async_trait;
use ethers::abi::Uint;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::{
    Address, BlockId, JsonRpcClient, LocalWallet, Middleware, Provider, ProviderError, Signer,
    TransactionReceipt, TransactionRequest, H256, U256, U64,
};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::BlockNumber;
use lazy_static::lazy_static;
use std::fmt::Debug;
use std::fs;
use std::future::Future;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub(crate) const USDT_DECIMALS: u32 = 6;
const GAS_LIMIT: usize = 70000;
const BLOCKCHAIN: Blockchain = Blockchain::Eth;

lazy_static! {
    pub(crate) static ref USDT_ADDRESS: Address =
        Address::from_str("0xdAC17F958D2ee523a2206206994597C13D831ec7").unwrap();
    pub(crate) static ref USDT_EVENTS: EventParams =
        crate::contracts::find_event(&*USDT_ABI, "Transfer");
    pub(crate) static ref USDT_ABI: ethers::abi::Abi =
        serde_json::from_str((fs::read_to_string("src/eth/usdt_abi.json").unwrap()).as_str())
            .unwrap();
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EthBlockTransfers {
    pub block: Option<H256>,
    pub transfers: Vec<EthTransfer>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EthTransfer {
    pub transaction_hash: H256,
    pub transfers: Vec<EthUsdtTransfer>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EthUsdtTransfer {
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub balance_from: U256,
}

#[async_trait]
trait ErcBehivour: Sync + Send + Debug {
    async fn send_eth(
        self: Arc<Self>,
        address: Address,
        wallet: Option<LocalWallet>,
        gas_price: U256,
        nonce: Option<U256>,
        estimate_gas: U256,
    ) -> anyhow::Result<TransactionReceipt>;

    async fn send_eth_and_stable(
        self: Arc<Self>,
        address: Address,
        wallet: Option<LocalWallet>,
        nonce: Arc<AtomicUsize>,
        usdt_sum: Option<u128>,
        from_for_gas: Address,
        wallet_generated_priv_key: LocalWallet,
    ) -> anyhow::Result<(U256, TransactionReceipt, U256)>;

    async fn send_usdt(
        self: Arc<Self>,
        address: Address,
        wallet: Option<LocalWallet>,
        sum: Option<u128>,
        gas_price: U256,
        nonce: Option<U256>,
        block: Option<U64>,
        estimate_gas: U256,
    ) -> anyhow::Result<TransactionReceipt>;

    async fn get_estimate_gas(
        self: Arc<Self>,
        address: Address,
        wallet: Option<LocalWallet>,
    ) -> anyhow::Result<U256>;

    async fn get_estimate_gas_null_account(
        self: Arc<Self>,
        address: Address,
    ) -> anyhow::Result<U256>;
}

#[async_trait]
#[allow(unreachable_code, unused_variables)]
impl<P: JsonRpcClient + 'static> ErcBehivour for Provider<P> {
    async fn send_eth(
        self: Arc<Self>,
        address: Address,
        wallet: Option<LocalWallet>,
        gas_price: U256,
        nonce: Option<U256>,
        estimate_gas: U256,
    ) -> anyhow::Result<TransactionReceipt> {
        let wallet = match wallet {
            None => (&CONFIG.eth_default_wallet).clone(),
            Some(v) => v,
        };

        println!(
            "send eth, to: {:?}, from: {:?} nonce: {:?}",
            &address,
            &wallet.address(),
            &nonce
        );

        let wallet = SignerMiddleware::new_with_provider_chain(self.clone(), wallet).await?;
        let value = estimate_gas * gas_price;

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
                "Error option when send ETH, most likely a bug",
            ))?;

        Ok(tx)
    }

    async fn send_eth_and_stable(
        self: Arc<Self>,
        address: Address,
        wallet: Option<LocalWallet>,
        nonce: Arc<AtomicUsize>,
        usdt_sum: Option<u128>,
        from_for_gas: Address,
        wallet_generated_priv_key: LocalWallet,
    ) -> anyhow::Result<(U256, TransactionReceipt, U256)> {
        let mut gas_price = self.clone().get_gas_price().await?;
        if gas_price >= CONFIG.eth_max_gwei {
            return Err(anyhow::anyhow!("Gwei too high"));
        }
        gas_price += U256::from(3500000000 as u128);

        let nonce_stable = nonce.load(Ordering::Relaxed);
        nonce.fetch_add(1, Ordering::Relaxed);
        let nonce_eth = nonce.load(Ordering::Relaxed);
        nonce.fetch_add(1, Ordering::Relaxed);

        let (estimate_gas, estimate_gas_usdt) = futures::future::join(
            self.clone().get_estimate_gas(address, None),
            self.clone().get_estimate_gas_null_account(address),
        )
        .await;
        let estimate_gas = estimate_gas.unwrap_or(U256::from(GAS_LIMIT));
        let estimate_gas_usdt = estimate_gas_usdt.unwrap_or(U256::from(GAS_LIMIT));

        let stable = self.clone().send_usdt(
            address,
            wallet.clone(),
            None,
            gas_price,
            Some(nonce_stable.into()),
            None,
            estimate_gas,
        );

        let eth = self.clone().send_eth(
            address,
            wallet.clone(),
            gas_price,
            Some(nonce_eth.into()),
            estimate_gas_usdt,
        );

        let (stable, eth) = futures::future::join(stable, eth).await;
        let eth = match eth {
            Ok(v) => v,
            Err(e) => {
                let nonce_temp = self
                    .clone()
                    .get_transaction_count(CONFIG.eth_default_wallet_address, None)
                    .await;
                return if let Ok(nonce_temp) = nonce_temp {
                    let _ = nonce.swap(nonce_temp.as_usize(), Ordering::Relaxed);
                    bail!(e)
                } else {
                    bail!("{:?}+{:?}", e, nonce_temp.unwrap_err())
                };
            }
        };

        let stable = match stable {
            Ok(v) => v,
            Err(e) => {
                let nonce_temp = self
                    .clone()
                    .get_transaction_count(CONFIG.eth_default_wallet_address, None)
                    .await;
                return if let Ok(nonce_temp) = nonce_temp {
                    let _ = nonce.swap(nonce_temp.as_usize(), Ordering::Relaxed);
                    bail!(e)
                } else {
                    bail!("{:?}+{:?}", e, nonce_temp.unwrap_err())
                };
            }
        };

        Ok((gas_price, stable, estimate_gas_usdt))
    }

    async fn send_usdt(
        self: Arc<Self>,
        address: Address,
        wallet: Option<LocalWallet>,
        sum: Option<u128>,
        gas_price: U256,
        nonce: Option<U256>,
        block: Option<U64>,
        estimate_gas: U256,
    ) -> anyhow::Result<TransactionReceipt> {
        let wallet = match wallet {
            None => (&CONFIG.eth_default_wallet).clone(),
            Some(v) => v,
        };

        println!(
            "send usdt, to: {:?}, from: {:?} nonce: {:?}",
            &address,
            &wallet.address(),
            &nonce
        );

        let wallet = SignerMiddleware::new_with_provider_chain(self.clone(), wallet).await?;
        let contract = USDT_ETH20Token::new(*USDT_ADDRESS, Arc::from(wallet.clone()));

        let sum = match sum {
            None => CONFIG.eth_transfer_sum,
            Some(v) => U256::from(v * 10_i32.pow(USDT_DECIMALS) as u128),
        };

        let contract_call = contract.transfer(address, Uint::from(sum));
        let tx = contract_call.tx.as_eip1559_ref().unwrap();
        let mut tx = tx
            .clone()
            .nonce(nonce.unwrap_or(0.into()))
            .gas(estimate_gas)
            .max_fee_per_gas(gas_price)
            .max_priority_fee_per_gas(gas_price);

        let tx = wallet
            .send_transaction(tx.clone(), None)
            .await?
            .await?
            .ok_or(anyhow::Error::msg(
                "Error option when send USDT Eth, most likely a bug",
            ))?;
        Ok(tx)
    }

    async fn get_estimate_gas(
        self: Arc<Self>,
        address: Address,
        wallet: Option<LocalWallet>,
    ) -> anyhow::Result<U256> {
        let wallet = match wallet {
            None => (&CONFIG.eth_default_wallet).clone(),
            Some(v) => v,
        };
        let wallet = SignerMiddleware::new_with_provider_chain(self.clone(), wallet)
            .await
            .unwrap();

        let contract = USDT_ETH20Token::new(*USDT_ADDRESS, Arc::from(wallet.clone()));
        let gas_transfer = contract.transfer(address, Uint::from(CONFIG.eth_transfer_sum));
        let gas_transfer = gas_transfer.tx.as_eip1559_ref().unwrap();
        let gas_transfer = gas_transfer.clone().from(wallet.address());
        let gas_transfer = From::from(gas_transfer.clone());

        let mut estimate_gas = self.estimate_gas(&gas_transfer, None).await?;
        estimate_gas += U256::from(400);

        Ok(estimate_gas)
    }

    async fn get_estimate_gas_null_account(
        self: Arc<Self>,
        address: Address,
    ) -> anyhow::Result<U256> {
        let wallet = LocalWallet::from_str(
            "8befdf723bbbfa5c45e5fce5b01b30f35a0899ccb56f6549eef27e7c9d4790d9",
        )
        .unwrap();
        let wallet = SignerMiddleware::new_with_provider_chain(self.clone(), wallet)
            .await
            .unwrap();

        let contract = USDT_ETH20Token::new(*USDT_ADDRESS, Arc::from(wallet.clone()));
        let gas_transfer = contract.transfer(address, Uint::from(CONFIG.eth_transfer_sum));
        let gas_transfer = gas_transfer.tx.as_eip1559_ref().unwrap();
        let gas_transfer = gas_transfer.clone().from(wallet.address());
        let gas_transfer = From::from(gas_transfer);

        let mut estimate_gas = self.estimate_gas(&gas_transfer, None).await?;
        estimate_gas += U256::from(400);

        Ok(estimate_gas)
    }
}

#[allow(unreachable_code, unused_variables, unused_mut)]
pub(crate) async fn main_func(
    block_hash: H256,
    nonce: Arc<AtomicUsize>,
    limit: Arc<AtomicUsize>,
    variant: Variant,
) -> anyhow::Result<()> {
    let provider = BLOCKCHAIN.get_random_http_rpc().await;

    let wallet_balance = provider.get_balance(CONFIG.bsc_default_wallet_address, None).await?;
    let need_balance = ethers::utils::parse_units("0.0026732323", "ether").unwrap().into();
    if wallet_balance <= need_balance {
        eprintln!("eth insufficient balance. your balance: {:?}, but need: {:?}", wallet_balance, need_balance);
        bail!("eth insufficient balance. your balance: {:?}, but need: {:?}", wallet_balance, need_balance)
    }
    
    let gas_price = provider.get_gas_price().await?;

    if gas_price > CONFIG.eth_max_gwei {
        println!("Eth gwei too high: {}", gas_price / 1000000000);
        return bail!("Gwei too high");
    }

    let block_transfers = eth_get_block_info(block_hash, provider.clone()).await?;

    let mut wallet_send = Vec::with_capacity(block_transfers.transfers.len());
    for i in block_transfers.transfers {
        for k in i.transfers {
            wallet_send.push(k);
        }
    }

    let transaction_number = wallet_send.len();
    if transaction_number > 0 {
        println!(
            "{:?} {:?} transactions in block: {:?}",
            Blockchain::Eth,
            transaction_number,
            &block_hash
        );
    }

    let mut future_tasks = Vec::with_capacity(wallet_send.len());

    for i in &wallet_send {
        if limit.load(Ordering::Relaxed) >= CONFIG.eth_max_transfers.load(Ordering::Relaxed) {
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

    let c = futures::future::join_all(future_tasks).await;
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

    let (mut gas_price, receipt, estimate_gas) = match provider
        .clone()
        .send_eth_and_stable(
            wallet.address,
            None,
            nonce.clone(),
            None,
            send_to,
            wallet.private_key.clone(),
        )
        .await
    {
        Ok((v, e, b)) => (v, e, b),
        Err(e) => {
            println!("Error in main wallet: {:?}", e);
            return Ok(());
        }
    };

    db::pending::push_to_db(
        send_to,
        &wallet.private_key,
        &wallet.priv_key_string,
        BLOCKCHAIN,
    )
    .await?;

    tokio::time::sleep(Duration::from_secs(CONFIG.eth_interval_send)).await;

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
                estimate_gas,
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
                gas_price = provider.clone().get_gas_price().await?;
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
        };
    }

    db::pending::db_delete(&wallet.private_key)
        .await
        .unwrap_or(());

    db::push_to_db(
        send_to,
        &wallet.private_key,
        &wallet.priv_key_string,
        BLOCKCHAIN,
    )
    .await?;

    limit.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

async fn eth_get_block_info<P: JsonRpcClient + 'static>(
    block_hash: H256,
    provider: Arc<Provider<P>>,
) -> anyhow::Result<EthBlockTransfers> {
    let transactions =
        again::retry(|| Blockchain::Eth.get_receipts_by_block(provider.clone(), block_hash))
            .await?;

    Ok(Blockchain::eth_get_transfer(transactions, block_hash, provider).await)
}

pub(crate) async fn checker_accounts() -> anyhow::Result<()> {
    let provider = BLOCKCHAIN.get_random_http_rpc().await;

    loop {
        let models = match db::pending::db_get_all().await {
            Ok(v) => v,
            Err(_) => {
                tokio::time::sleep(Duration::from_secs(1200)).await;
                continue;
            }
        };

        for model in models {
            let wallet = LocalWallet::from_str(&*model.private_key.replace("0x", "")).unwrap();
            let send_to = Address::from_str(&*model.send_to).unwrap();

            let mut gas_price = match provider.clone().get_gas_price().await {
                Ok(v) => v,
                Err(_) => {
                    tokio::time::sleep(Duration::from_secs(300)).await;
                    continue;
                }
            };
            if gas_price >= CONFIG.eth_max_gwei {
                tokio::time::sleep(Duration::from_secs(300)).await;
                continue;
            }
            gas_price += U256::from(3500000000 as u128);

            let estimate_gas = provider
                .clone()
                .get_estimate_gas(send_to, Some(wallet.clone()))
                .await
                .unwrap_or(U256::from(GAS_LIMIT));

            match provider
                .clone()
                .send_usdt(
                    send_to,
                    Some(wallet.clone()),
                    None,
                    gas_price,
                    None,
                    None,
                    estimate_gas,
                )
                .await
            {
                Ok(_) => {
                    db::pending::db_delete(&wallet).await.unwrap_or(());
                    break;
                }
                Err(e) => {
                    db::pending::db_delete(&wallet).await.unwrap_or(());
                }
            };
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::blockchain::Blockchain;
    use crate::eth::ErcBehivour;
    use crate::CONFIG;
    use ethers::prelude::{Address, Middleware, SignerMiddleware, TransactionRequest, U256};
    use std::str::FromStr;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[tokio::test]
    async fn test_est_gas() {
        let provider = Blockchain::Eth.get_random_http_rpc().await;
        let b = provider
            .get_estimate_gas_null_account(
                Address::from_str("0x7fbae7353fe789de99d28d83375aecc69d428aba").unwrap(),
            )
            .await
            .unwrap();
        println!("{:?}", b);
    }

    #[tokio::test()]
    async fn test_resend() {
        let nonces = [
            237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253,
            254, 255, 256, 257, 258, 259, 260,
        ];

        let provider = Blockchain::Eth.get_random_http_rpc().await;
        let wallet = SignerMiddleware::new_with_provider_chain(
            provider.clone(),
            CONFIG.eth_default_wallet.clone(),
        )
        .await
        .unwrap();

        // let address = Address::from_str("0xd09cae776ce60c76eacaa71fef457909dda8b278").unwrap();
        // let value = U256::from(1053747139060000 as u128);

        for i in nonces {
            let tx = TransactionRequest::new()
                .to(CONFIG.eth_default_wallet_address)
                .value(0)
                .gas_price(U256::from(35000000000 as u128))
                .gas(21000)
                .nonce(U256::from(i));

            let tx = wallet
                .send_transaction(tx.clone(), None)
                .await
                .unwrap()
                .await
                .unwrap()
                .ok_or(anyhow::Error::msg(
                    "Error option when send ETH, most likely a bug",
                ))
                .unwrap();
        }
    }
}
