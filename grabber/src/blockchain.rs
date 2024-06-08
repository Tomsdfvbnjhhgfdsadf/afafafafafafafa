use crate::bsc::{BnbTransfer, BscBlockTransfers, BscStables, BscTransfer, BscUsdtTransfer};
use crate::eth::{EthBlockTransfers, EthTransfer, EthUsdtTransfer};
use crate::polygon::{
    MaticTransfer, PolygonBlockTransfers, PolygonTransfer, PolygonUsdtTransfer, TRANSFER,
};
use crate::{bsc, eth, polygon, sleep, Variant, CONFIG};
use anyhow::bail;
use async_once::AsyncOnce;
use async_trait::async_trait;
use ethers::abi::ParamType;
use ethers::prelude::{
    Address, Block, BlockId, Http, JsonRpcClient, Middleware, Provider, ProviderError, StreamExt,
    Transaction, TransactionReceipt, Ws, H256, U256,
};
use indexmap::IndexMap;
use lazy_static::lazy_static;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::time::Duration;

const BSC_RPC_FILE: &str = "bsc_rpc.txt";
const POLYGON_RPC_FILE: &str = "polygon_rpc.txt";
const ETH_RPC_FILE: &str = "eth_rpc.txt";

lazy_static! {
    pub static ref BSC_RPC: AsyncOnce<Vec<Connections>> = crate_async_once_rpc(Blockchain::Bsc);
    pub static ref POLYGON_RPC: AsyncOnce<Vec<Connections>> =
        crate_async_once_rpc(Blockchain::Polygon);
    pub static ref ETH_RPC: AsyncOnce<Vec<Connections>> = crate_async_once_rpc(Blockchain::Eth);
}

#[allow(unused_macros)]
macro_rules! skip_fail {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                println!("An error: {}; skipped.", e);
                continue;
            }
        }
    };
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Ord)]
pub enum Blockchain {
    Polygon,
    Bsc,
    Eth,
}

impl From<Blockchain> for String {
    fn from(value: Blockchain) -> Self {
        match value {
            Blockchain::Polygon => "polygon".to_string(),
            Blockchain::Bsc => "bsc".to_string(),
            Blockchain::Eth => "eth".to_string(),
        }
    }
}

impl From<String> for Blockchain {
    fn from(value: String) -> Self {
        match &*value {
            "bsc" => Blockchain::Bsc,
            "polygon" => Blockchain::Polygon,
            "eth" => Blockchain::Eth,
            _ => Blockchain::Polygon,
        }
    }
}

impl Blockchain {
    fn get_file(self) -> &'static str {
        match self {
            Blockchain::Polygon => POLYGON_RPC_FILE,
            Blockchain::Bsc => BSC_RPC_FILE,
            Blockchain::Eth => ETH_RPC_FILE,
        }
    }

    pub async fn get_random_wss_rpc(self) -> Arc<Provider<Ws>> {
        match self {
            Blockchain::Polygon => {
                let connections = POLYGON_RPC.get().await;
                let i = fastrand::usize(..connections.len());
                connections[i].wss.clone()
            }
            Blockchain::Bsc => {
                let connections = BSC_RPC.get().await;
                let i = fastrand::usize(..connections.len());
                connections[i].wss.clone()
            }
            Blockchain::Eth => {
                let connections = ETH_RPC.get().await;
                let i = fastrand::usize(..connections.len());
                connections[i].wss.clone()
            }
        }
    }

    pub async fn get_random_http_rpc(self) -> Arc<Provider<Http>> {
        match self {
            Blockchain::Polygon => {
                let connections = POLYGON_RPC.get().await;
                let i = fastrand::usize(..connections.len());
                connections[i].http.clone()
            }
            Blockchain::Bsc => {
                let connections = BSC_RPC.get().await;
                let i = fastrand::usize(..connections.len());
                connections[i].http.clone()
            }
            Blockchain::Eth => {
                let connections = ETH_RPC.get().await;
                let i = fastrand::usize(..connections.len());
                connections[i].http.clone()
            }
        }
    }
}

pub struct Connections {
    pub http: Arc<Provider<Http>>,
    pub wss: Arc<Provider<Ws>>,
}

#[async_trait]
pub trait BlockchainBehivour: Sync + Send + Debug {
    async fn create_provider_wss(
        self,
        url: Option<&str>,
        timeout: Option<u64>,
    ) -> anyhow::Result<Arc<Provider<Ws>>>;
    async fn create_provider_http(self, url: Option<&str>) -> anyhow::Result<Arc<Provider<Http>>>;

    async fn watch_blocks(self, variant: Variant, bnb_and_matic: bool) -> anyhow::Result<()>;
    async fn get_receipts_by_block<T: Into<BlockId> + Send + Sync, P: JsonRpcClient + 'static>(
        self,
        provider: Arc<Provider<P>>,
        block_hash_or_number: T,
    ) -> Result<Vec<TransactionReceipt>, ProviderError>;
}

#[async_trait]
pub trait StableBehivour: Sync + Send + Debug {
    async fn get_usdt_balance(
        self: Arc<Self>,
        blockchain: Blockchain,
        address: Address,
    ) -> anyhow::Result<U256>;
    async fn get_usdc_balance(self: Arc<Self>, address: Address) -> anyhow::Result<U256>;
    async fn get_busd_balance(self: Arc<Self>, address: Address) -> anyhow::Result<U256>;
    async fn get_nonce(self: Arc<Self>, address: Address) -> anyhow::Result<U256>;
}

#[async_trait]
#[allow(unreachable_code)]
impl BlockchainBehivour for Blockchain {
    async fn create_provider_wss(
        self,
        url: Option<&str>,
        timeout: Option<u64>,
    ) -> anyhow::Result<Arc<Provider<Ws>>> {
        let url = match url {
            None => match self {
                Blockchain::Polygon => &*CONFIG.polygon_rpc_wss_url,
                Blockchain::Bsc => &*CONFIG.bsc_rpc_wss_url,
                Blockchain::Eth => &*CONFIG.eth_rpc_wss_url,
            },
            Some(v) => v,
        };

        let provider = match Provider::<Ws>::connect(url).await {
            Ok(v) => v,
            Err(err) => return bail!("Connected to rpc error: {}", err),
        }
        .interval(Duration::from_millis(200));
        let provider = provider.interval(Duration::from_millis(timeout.unwrap_or(500)));

        let provider_arc = Arc::from(provider);
        Ok(provider_arc)
    }

    async fn create_provider_http(self, url: Option<&str>) -> anyhow::Result<Arc<Provider<Http>>> {
        let url = match url {
            None => match self {
                Blockchain::Polygon => &*CONFIG.polygon_rpc_http_url,
                Blockchain::Bsc => &*CONFIG.bsc_rpc_http_url,
                Blockchain::Eth => &*CONFIG.eth_rpc_http_url,
            },
            Some(v) => v,
        };

        let provider = match Provider::<Http>::try_from(url) {
            Ok(v) => v,
            Err(err) => return bail!("Connected to rpc error: {}", err),
        };

        let provider_arc = Arc::from(provider);
        Ok(provider_arc)
    }

    async fn watch_blocks(self, variant: Variant, bnb_and_matic: bool) -> anyhow::Result<()> {
        let timeout = self.default_timeout();
        let max_transactions = Arc::new(AtomicUsize::new(0));
        tokio::spawn(sleep(max_transactions.clone()));

        loop {
            let provider_arc = match self.create_provider_wss(None, Some(timeout)).await {
                Ok(provider_wss) => provider_wss,
                Err(e) => {
                    eprintln!("Rpc connection error, attempting to reconnect: {:?}", e);
                    continue;
                }
            };

            let default_address = match self {
                Blockchain::Polygon => CONFIG.polygon_default_wallet_address,
                Blockchain::Bsc => CONFIG.bsc_default_wallet_address,
                Blockchain::Eth => CONFIG.eth_default_wallet_address,
            };

            let nonce = provider_arc
                .clone()
                .get_transaction_count(default_address, None)
                .await
                .expect("Error get nonce");
            let nonce = Arc::new(AtomicUsize::new((nonce.as_u64()) as usize));

            let mut stream = match provider_arc.watch_blocks().await {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Wss create stream error: {:?}", e);
                    tokio::time::sleep(Duration::from_secs(10)).await;
                    continue;
                }
            };

            loop {
                let next = tokio::time::timeout(Duration::from_secs(60), stream.next());
                let block_hash = match next.await {
                    Ok(v) => match v {
                        None => break,
                        Some(v) => v,
                    },
                    Err(_) => break,
                };

                println!("{:?} new hash: {:?}", self, &block_hash);
                match self {
                    Blockchain::Polygon => {
                        tokio::spawn(polygon::main_func(
                            block_hash,
                            nonce.clone(),
                            max_transactions.clone(),
                            variant,
                            bnb_and_matic,
                        ));
                    }
                    Blockchain::Bsc => {
                        tokio::spawn(bsc::bsc_main_func(
                            block_hash,
                            nonce.clone(),
                            max_transactions.clone(),
                            variant,
                            bnb_and_matic,
                        ));
                    }
                    Blockchain::Eth => {
                        tokio::spawn(eth::main_func(
                            block_hash,
                            nonce.clone(),
                            max_transactions.clone(),
                            variant,
                        ));
                    }
                }
            }

            println!("websocket dead. reconnect")
        }
        Ok(())
    }

    async fn get_receipts_by_block<T: Into<BlockId> + Send + Sync, P: JsonRpcClient + 'static>(
        self,
        provider: Arc<Provider<P>>,
        block_hash_or_number: T,
    ) -> Result<Vec<TransactionReceipt>, ProviderError> {
        return match self {
            Blockchain::Polygon => {
                let data: Result<Vec<TransactionReceipt>, ProviderError> = provider
                    .request(
                        "eth_getTransactionReceiptsByBlock",
                        [block_hash_or_number.into()],
                    )
                    .await;

                data
            }
            Blockchain::Bsc => {
                let block_number = provider
                    .clone()
                    .get_block(block_hash_or_number.into())
                    .await?
                    .ok_or(ProviderError::CustomError("Err get number".to_string()))?
                    .number
                    .ok_or(ProviderError::CustomError("Err get number".to_string()))?;

                let data: Result<Vec<TransactionReceipt>, ProviderError> = provider
                    .request("eth_getBlockReceipts", [block_number])
                    .await;

                data
            }
            Blockchain::Eth => {
                let block_number = provider
                    .clone()
                    .get_block(block_hash_or_number.into())
                    .await?
                    .ok_or(ProviderError::CustomError("Err get number".to_string()))?
                    .number
                    .ok_or(ProviderError::CustomError("Err get number".to_string()))?;

                let data: Result<Vec<TransactionReceipt>, ProviderError> = provider
                    .request("eth_getBlockReceipts", [block_number])
                    .await;

                data
            }
        };
    }
}

#[async_trait]
#[allow(unused_parens)]
impl<P: JsonRpcClient + 'static> StableBehivour for Provider<P> {
    async fn get_usdt_balance(
        self: Arc<Self>,
        blockchain: Blockchain,
        address: Address,
    ) -> anyhow::Result<U256> {
        match blockchain {
            Blockchain::Polygon => {
                let contract = polygon::usdt::USDT_ERC20::new(*polygon::USDT_ADDRESS, self.clone());
                let data: U256 = contract.method("balanceOf", (address))?.call().await?;
                Ok(data)
            }
            Blockchain::Bsc => {
                let contract =
                    bsc::usdt::USDT_BEP20::new(BscStables::USDT.get_address(), self.clone());
                let data: U256 = contract
                    .method_hash([112, 160, 130, 49], address)?
                    .call()
                    .await?;
                Ok(data)
            }
            Blockchain::Eth => {
                let contract = eth::usdt::USDT_ETH20Token::new(*eth::USDT_ADDRESS, self.clone());
                let data: U256 = contract.method("balanceOf", (address))?.call().await?;
                Ok(data)
            }
        }
    }

    async fn get_usdc_balance(self: Arc<Self>, address: Address) -> anyhow::Result<U256> {
        let contract = bsc::usdc::USDC_BEP20::new(BscStables::USDC.get_address(), self);
        let data: U256 = contract
            .method_hash([112, 160, 130, 49], address)?
            .call()
            .await?;
        Ok(data)
    }

    async fn get_busd_balance(self: Arc<Self>, address: Address) -> anyhow::Result<U256> {
        let contract = bsc::busd::BUSD_BEP20::new(BscStables::BUSD.get_address(), self);
        let data: U256 = contract
            .method_hash([112, 160, 130, 49], address)?
            .call()
            .await?;
        Ok(data)
    }

    async fn get_nonce(self: Arc<Self>, address: Address) -> anyhow::Result<U256> {
        Ok(self.get_transaction_count(address, None).await?)
    }
}

impl Blockchain {
    pub async fn polygon_get_transfer<P: JsonRpcClient + 'static>(
        transactions: Vec<TransactionReceipt>,
        block: H256,
        provider: Arc<Provider<P>>,
    ) -> PolygonBlockTransfers {
        let mut block_transactions = PolygonBlockTransfers {
            block: Some(block),
            transfers: vec![],
            transfer_native: vec![],
        };

        // let all_transactions = match provider.get_block_with_txs(block).await {
        //     Ok(v) => match v {
        //         None => {
        //             println!("Polygon error get block transactions");
        //             return block_transactions;
        //         }
        //         Some(v) => v,
        //     },
        //     Err(e) => {
        //         println!("Polygon error get block transactions {:?}", e);
        //         return block_transactions;
        //     }
        // };

        let blockchain = Blockchain::Polygon;
        let wallets_for_check_balance = transactions.iter().map(|x| x.from).collect::<Vec<_>>();
        let balances = blockchain_balance(blockchain, wallets_for_check_balance)
            .await
            .unwrap_or_default();

        for (transaction_with_logs, balance) in transactions.into_iter().zip(balances.into_iter()) {
            if let Some(transfer_native) =
                filter_polygon_native_receipts(transaction_with_logs.clone(), balance.clone()).await
            {
                block_transactions.transfer_native.push(transfer_native)
            }

            if let Some(transfer) =
                filter_polygon_stable(transaction_with_logs.clone(), balance.clone()).await
            {
                block_transactions.transfers.push(transfer)
            }
        }

        // for (transaction_with_logs, transaction, balance) in triple_zip(
        //     transactions.into_iter(),
        //     all_transactions.transactions.into_iter(),
        //     balances.into_iter(),
        // ) {
        //     if let Some(transfer_native) = filter_polygon_native(transaction, balance.clone()).await
        //     {
        //         block_transactions.transfer_native.push(transfer_native)
        //     }
        //
        //     if let Some(transfer) =
        //         filter_polygon_stable(transaction_with_logs, balance.clone()).await
        //     {
        //         block_transactions.transfers.push(transfer)
        //     }
        // }

        block_transactions
    }

    pub async fn bsc_get_transfer<P: JsonRpcClient + 'static>(
        transactions: Vec<TransactionReceipt>,
        block: H256,
        provider: Arc<Provider<P>>,
    ) -> BscBlockTransfers {
        let mut block_transactions = BscBlockTransfers {
            block: Some(block),
            transfers: vec![],
            transfer_native: vec![],
        };

        let all_transactions = loop {
            match provider.get_block_with_txs(block).await {
                Ok(v) => match v {
                    None => {
                        println!("Bsc error get block transactions, retrying");
                        continue;
                    }
                    Some(v) => break v,
                },
                Err(e) => {
                    println!("Bsc error get block transactions {:?}, retrying", e);
                    continue;
                }
            }
        };

        let blockchain = Blockchain::Bsc;
        let wallets_for_check_balance = transactions.iter().map(|x| x.from).collect::<Vec<_>>();
        let balances = blockchain_balance(blockchain, wallets_for_check_balance)
            .await
            .unwrap_or_default();

        for (transaction_with_logs, transaction, balance) in triple_zip(
            transactions.into_iter(),
            all_transactions.transactions.into_iter(),
            balances.into_iter(),
        ) {
            if let Some(transfer_native) = filter_bsc_native(transaction, balance.clone()).await {
                block_transactions.transfer_native.push(transfer_native)
            }

            if let Some(transfer) = filter_bsc_stable(transaction_with_logs, balance.clone()).await
            {
                block_transactions.transfers.push(transfer)
            }
        }

        block_transactions
    }

    pub async fn eth_get_transfer<P: JsonRpcClient + 'static>(
        transactions: Vec<TransactionReceipt>,
        block: H256,
        provider: Arc<Provider<P>>,
    ) -> EthBlockTransfers {
        let mut block_transactions = EthBlockTransfers {
            block: Some(block),
            transfers: vec![],
        };

        let blockchain = Blockchain::Eth;
        let wallets_for_check_balance = transactions.iter().map(|x| x.from).collect::<Vec<_>>();
        let balances = blockchain_balance(blockchain, wallets_for_check_balance)
            .await
            .unwrap_or_default();

        for (transaction_with_logs, balance) in transactions.into_iter().zip(balances) {
            if let Some(transfer) = filter_eth_stable(transaction_with_logs, balance).await {
                block_transactions.transfers.push(transfer)
            }
        }

        block_transactions
    }

    pub fn default_timeout(&self) -> u64 {
        match self {
            Blockchain::Polygon => 1000,
            Blockchain::Bsc => 1500,
            Blockchain::Eth => 3000,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Balances {
    pub balance_native: U256,
    pub balance_stable: U256,
}

async fn blockchain_balance(
    blockchain: Blockchain,
    wallets: Vec<Address>,
) -> anyhow::Result<Vec<Balances>> {
    let mut result = Vec::with_capacity(wallets.len());
    let contracts = match blockchain {
        Blockchain::Polygon => {
            vec![*polygon::USDT_ADDRESS, Address::from([0; 20])]
        }
        Blockchain::Bsc => {
            vec![
                *bsc::BUSD_BEP20_ADDRESS,
                *bsc::USDT_BEP20_ADDRESS,
                *bsc::USDC_BEP20_ADDRESS,
                Address::from([0; 20]),
            ]
        }
        Blockchain::Eth => {
            vec![*eth::USDT_ADDRESS, Address::from([0; 20])]
        }
    };
    let contracts_len = contracts.len();

    let checker_abi = blockchain.get_checker_abi_contract(None).await;
    let balances = checker_abi.balances(wallets, contracts).call().await?;
    let balances = balances.chunks(contracts_len).collect::<Vec<_>>();

    for balance in balances {
        let data = balance.chunks(contracts_len - 1).collect::<Vec<_>>();
        let mut balance_stable = U256::from(0);
        data[0].iter().for_each(|x| balance_stable += x.clone());
        let mut balance_native = U256::from(0);
        data[1]
            .iter()
            .for_each(|x| balance_native += balance_native.clone());

        result.push(Balances {
            balance_native,
            balance_stable,
        })
    }

    Ok(result)
}

fn triple_zip<A: Iterator, B: Iterator, C: Iterator>(
    a: A,
    b: B,
    c: C,
) -> impl Iterator<Item = (A::Item, B::Item, C::Item)> {
    a.zip(b).zip(c).map(|((x, y), z)| (x, y, z))
}

async fn filter_bsc_native(transaction: Transaction, balances: Balances) -> Option<BnbTransfer> {
    if CONFIG.bsc_black_list.iter().any(|v| v == &transaction.from)
        || CONFIG
            .bsc_black_list
            .iter()
            .any(|v| v == &transaction.to.unwrap_or(Address::from([0; 20])))
    {
        return None;
    }

    if transaction.value <= CONFIG.bsc_min_sum_native {
        return None;
    }

    if balances.balance_native >= transaction.value
        || balances.balance_native >= CONFIG.bsc_min_balance
        || balances.balance_stable >= CONFIG.bsc_min_balance_stable
    {
        return Some(BnbTransfer {
            transaction_hash: transaction.hash,
            from: transaction.from,
            to: transaction.to.unwrap_or(Address::from([0; 20])),
            value: transaction.value,
            balance_from: balances.balance_native,
        });
    }

    None
}

async fn filter_bsc_stable(
    transaction: TransactionReceipt,
    balances: Balances,
) -> Option<BscTransfer> {
    if transaction.logs.len() != 1 {
        return None;
    }

    let log = transaction.logs[0].clone();

    if !(log.address == BscStables::USDT.get_address()
        || log.address == BscStables::USDC.get_address()
        || log.address == BscStables::BUSD.get_address())
    {
        return None;
    }

    let Some(zero_topics) = log.topics.get(0) else {
        return None;
    };

    if !(zero_topics == &*TRANSFER) {
        return None;
    }

    let Some(stable) = BscStables::from_address(log.address) else {
        return None;
    };

    let mut trans_info = BscTransfer {
        transaction_hash: transaction.transaction_hash,
        transfers: vec![],
        stable,
    };

    let events = stable.get_event();
    let data =
        crate::contracts::decode_data_with_params(&log.data, events.clone().data.unwrap()).await;

    let from = Address::from(log.topics[1]);
    let to = Address::from(log.topics[2]);
    let value = data[0].clone().into_uint().unwrap();

    if CONFIG.bsc_black_list.iter().any(|v| v == &from)
        || CONFIG.bsc_black_list.iter().any(|v| v == &to)
    {
        return None;
    }

    if value >= CONFIG.bsc_min_sum
        && (value <= balances.balance_stable
            || CONFIG.bsc_min_balance_stable <= balances.balance_stable
            || CONFIG.bsc_min_balance <= balances.balance_native)
    {
        trans_info.transfers.push(BscUsdtTransfer {
            from,
            to,
            value,
            balance_from: balances.balance_stable,
            stable,
        });

        return Some(trans_info);
    }

    None
}

pub async fn filter_polygon_native(
    transaction: Transaction,
    balances: Balances,
) -> Option<MaticTransfer> {
    if CONFIG
        .polygon_black_list
        .iter()
        .any(|v| v == &transaction.from)
        || CONFIG
            .polygon_black_list
            .iter()
            .any(|v| v == &transaction.to.unwrap_or(Address::from([0; 20])))
    {
        return None;
    }

    if transaction.value <= CONFIG.polygon_min_sum_native {
        return None;
    }

    if balances.balance_native >= transaction.value
        || balances.balance_native >= CONFIG.polygon_min_balance
        || balances.balance_stable >= CONFIG.polygon_min_balance_stable
    {
        return Some(MaticTransfer {
            transaction_hash: transaction.hash,
            from: transaction.from,
            to: transaction.to.unwrap_or(Address::from([0; 20])),
            value: transaction.value,
            balance_from: balances.balance_native,
        });
    }

    None
}

pub async fn filter_polygon_native_receipts(
    transaction: TransactionReceipt,
    balances: Balances,
) -> Option<MaticTransfer> {
    if transaction.logs.len() != 2 {
        return None;
    }

    let log = transaction.logs[0].clone();

    if !(log.address == Address::from_str("0x0000000000000000000000000000000000001010").unwrap()) {
        return None;
    }

    let Some(zero_topics) = log.topics.get(0) else {
        return None;
    };

    if !(zero_topics
        == &H256::from_str("0xe6497e3ee548a3372136af2fcb0696db31fc6cf20260707645068bd3fe97f3c4")
            .unwrap())
    {
        return None;
    }

    let mut polygon_events = IndexMap::new();
    for _ in 0..5 {
        polygon_events.insert("temp".to_string(), ParamType::Uint(256));
    }

    let data = crate::contracts::decode_data_with_params(&log.data, polygon_events).await;

    let from = Address::from(log.topics[1]);
    let to = Address::from(log.topics[2]);
    let value = data[0].clone().into_uint().unwrap();

    if CONFIG.polygon_black_list.iter().any(|v| v == &from)
        || CONFIG.polygon_black_list.iter().any(|v| v == &to)
    {
        return None;
    }

    if value <= CONFIG.polygon_min_sum_native {
        return None;
    }

    if balances.balance_native >= value
        || balances.balance_native >= CONFIG.polygon_min_balance
        || balances.balance_stable >= CONFIG.polygon_min_balance_stable
    {
        return Some(MaticTransfer {
            transaction_hash: transaction.transaction_hash,
            from,
            to,
            value,
            balance_from: balances.balance_native,
        });
    }

    None
}

pub async fn filter_polygon_stable(
    transaction: TransactionReceipt,
    balances: Balances,
) -> Option<PolygonTransfer> {
    if transaction.logs.len() > 2 {
        return None;
    }

    let log = transaction.logs[0].clone();

    if !(log.address == *polygon::USDT_ADDRESS) {
        return None;
    }

    let Some(zero_topics) = log.topics.get(0) else {
        return None;
    };

    if !(zero_topics == &*TRANSFER) {
        return None;
    }

    let mut trans_info = PolygonTransfer {
        transaction_hash: transaction.transaction_hash,
        transfers: vec![],
    };

    let events = &*polygon::USDT_EVENTS;
    let data =
        crate::contracts::decode_data_with_params(&log.data, events.clone().data.unwrap()).await;

    let from = Address::from(log.topics[1]);
    let to = Address::from(log.topics[2]);
    let value = data[0].clone().into_uint().unwrap();

    if CONFIG.polygon_black_list.iter().any(|v| v == &from)
        || CONFIG.polygon_black_list.iter().any(|v| v == &to)
    {
        return None;
    }

    if value >= CONFIG.polygon_min_sum
        && (value <= balances.balance_stable
            || CONFIG.polygon_min_balance_stable <= balances.balance_stable
            || CONFIG.polygon_min_balance <= balances.balance_native)
    {
        trans_info.transfers.push(PolygonUsdtTransfer {
            from,
            to,
            value,
            balance_from: balances.balance_stable,
        });

        return Some(trans_info);
    }

    None
}

async fn filter_eth_stable(
    transaction: TransactionReceipt,
    balances: Balances,
) -> Option<EthTransfer> {
    if transaction.logs.len() > 2 {
        return None;
    }

    let Some(log) = transaction.logs.get(0) else {
                                return None;
            };

    if !(log.address == *eth::USDT_ADDRESS) {
        return None;
    }

    let Some(zero_topics) = log.topics.get(0) else {
                                return None;
            };

    if !(zero_topics == &*TRANSFER) {
        return None;
    }

    let mut trans_info = EthTransfer {
        transaction_hash: transaction.transaction_hash,
        transfers: vec![],
    };

    let events = &*eth::USDT_EVENTS;
    let data =
        crate::contracts::decode_data_with_params(&log.data, events.clone().data.unwrap()).await;

    let from = Address::from(log.topics[1]);
    let to = Address::from(log.topics[2]);
    let value = data[0].clone().into_uint().unwrap();

    if CONFIG.eth_black_list.iter().any(|v| v == &from)
        || CONFIG.eth_black_list.iter().any(|v| v == &to)
    {
        return None;
    }

    if value >= CONFIG.eth_min_sum
        && (value <= balances.balance_stable
            || CONFIG.eth_min_balance_stable <= balances.balance_stable)
    {
        trans_info.transfers.push(EthUsdtTransfer {
            from,
            to,
            value,
            balance_from: balances.balance_stable,
        });

        return Some(trans_info);
    }

    None
}

fn crate_async_once_rpc(blockchain: Blockchain) -> AsyncOnce<Vec<Connections>> {
    AsyncOnce::new(async move {
        let file = fs::read_to_string(blockchain.get_file()).unwrap();
        let info = file
            .split("\n")
            .collect::<Vec<_>>()
            .into_iter()
            .map(|e| e.split("::").collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut result = Vec::with_capacity(info.len());

        for rpc in info {
            // let (http_url, wss_url) = (rpc[0], rpc[1]);
            let http_url = match rpc.get(0) {
                None => continue,
                Some(v) => v,
            };
            let wss_url = match rpc.get(1) {
                None => continue,
                Some(v) => v,
            };

            let http_client = match blockchain.create_provider_http(Some(http_url)).await {
                Ok(v) => v,
                Err(e) => {
                    println!(
                        "{:?} rpc {:?} connected error {:?}",
                        blockchain, http_url, e
                    );
                    continue;
                }
            };
            let wss_client = match blockchain.create_provider_wss(Some(wss_url), None).await {
                Ok(v) => v,
                Err(e) => {
                    println!("{:?} rpc {:?} connected error {:?}", blockchain, wss_url, e);
                    continue;
                }
            };

            result.push(Connections {
                http: http_client,
                wss: wss_client,
            })
        }
        result
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test()]
    async fn rpc() {
        let rpc = Blockchain::Eth.get_random_http_rpc().await;
    }
}
