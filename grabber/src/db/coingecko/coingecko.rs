use crate::db;
use async_once::AsyncOnce;
use ethers::prelude::{Address, Signer};
use ethers::signers::LocalWallet;
use lazy_static::lazy_static;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::{any, OnConflict};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveValue, ConnectOptions, Database, IntoActiveModel, QuerySelect, SelectModel};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use crate::blockchain::Blockchain;
use crate::polygon::checker::PROXY;
use db::coingecko::gecko_chain as GeckoChain;
use db::coingecko::gecko_list as GeckoList;

lazy_static! {
    pub static ref DB_GECKO: AsyncOnce<DatabaseConnection> =
        AsyncOnce::new(DB_GECKO::create_db_connection());
    pub static ref PATH_BALANCES: String = {
        let full_path = fs::canonicalize(PathBuf::from("src/db/coingecko.sqlite")).unwrap();
        let path = full_path.to_str().unwrap();
        format!("sqlite:{}", path)
    };
}

impl DB_GECKO {
    async fn get_connection() -> &'static DatabaseConnection {
        DB_GECKO.get().await
    }

    async fn create_db_connection() -> DatabaseConnection {
        let mut opt = ConnectOptions::new((&*PATH_BALANCES).clone());
        opt.max_connections(500).sqlx_logging(false);

        let db = Database::connect(opt)
            .await
            .expect("Database connection pool creation failed");

        db
    }
}

pub async fn gecko_updater() -> anyhow::Result<()> {
    loop {
        match update_all_gecko_list().await {
            Ok(_) => tokio::time::sleep(Duration::from_secs(60)).await,
            Err(e) => tokio::time::sleep(Duration::from_secs(120)).await,
        }
    }
}

pub async fn update_all_gecko_list() -> anyhow::Result<()> {
    let mut vec = Vec::with_capacity(1250);
    for i in 1..5 {
        db::coingecko::api::markets(None, Some(i))
            .await?
            .into_iter()
            .for_each(|x| vec.push(x));
    }

    let mut vec_res = Vec::with_capacity(vec.len());

    vec.into_iter().for_each(|x| {
        vec_res.push(GeckoList::Model {
            id: 0,
            gecko_id: x.id,
            symbol: x.symbol,
            rank: x.market_cap_rank as i32,
            price: x.current_price,
        })
    });

    push_vec_gecko_list(vec_res).await?;

    Ok(())
}

async fn push_gecko_list(data: GeckoList::Model) -> anyhow::Result<()> {
    let mut data = data.into_active_model();
    data.id = ActiveValue::NotSet;

    data.insert(DB_GECKO::get_connection().await).await?;

    Ok(())
}

async fn push_vec_gecko_list(data: Vec<GeckoList::Model>) -> anyhow::Result<()> {
    let on_conflict = OnConflict::columns(vec![GeckoList::Column::GeckoId])
        .update_columns(vec![GeckoList::Column::Price, GeckoList::Column::Rank])
        .clone();

    GeckoList::Entity::insert_many(
        data.into_iter()
            .map(|x| {
                let mut x = x.into_active_model();
                x.id = ActiveValue::NotSet;
                x
            })
            .collect::<Vec<_>>(),
    )
    .on_conflict(on_conflict.clone())
    .exec(DB_GECKO::get_connection().await)
    .await?;

    Ok(())
}

async fn get_all_gecko_list() -> anyhow::Result<Vec<GeckoList::Model>> {
    Ok(GeckoList::Entity::find()
        .all(DB_GECKO::get_connection().await)
        .await?)
}

pub async fn gecko_list_to_gecko_chain() -> anyhow::Result<()> {
    let gecko_list = get_all_gecko_list().await?;
    for gecko in gecko_list.chunks(PROXY.len()) {
        let mut num = 0;
        let mut futures = Vec::with_capacity(PROXY.len());

        for name in gecko.to_vec().iter() {
            futures.push(tokio::spawn(db::coingecko::api::coin_by_id(
                name.gecko_id.clone(),
                Some(&*PROXY[num]),
            )));
            num += 1;
        }

        let futures_results = futures::future::join_all(futures).await;
        let mut data = Vec::with_capacity(futures_results.len());
        for i in futures_results {
            if let Ok(value) = i {
                if let Ok(value) = value {
                    data.push(value)
                }
            }
        }

        let mut data_new = Vec::with_capacity(data.len() * 2);
        for i in data {
            for model in i.to_model() {
                data_new.push(model)
            }
        }

        push_vec_gecko_chain(data_new).await.unwrap();
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
    Ok(())
}

async fn push_gecko_chain(data: GeckoChain::Model) -> anyhow::Result<()> {
    let on_conflict = OnConflict::new().do_nothing().to_owned();
    let mut data = data.into_active_model();
    data.id = ActiveValue::NotSet;

    GeckoChain::Entity::insert(data)
        .on_conflict(on_conflict)
        .exec(DB_GECKO::get_connection().await)
        .await?;

    Ok(())
}

async fn push_vec_gecko_chain(data: Vec<GeckoChain::Model>) -> anyhow::Result<()> {
    let on_conflict = OnConflict::new().do_nothing().to_owned();

    GeckoChain::Entity::insert_many(data.into_iter().map(|x| {
        let mut data = x.into_active_model();
        data.id = ActiveValue::NotSet;
        data
    }))
    .on_conflict(on_conflict)
    .exec(DB_GECKO::get_connection().await)
    .await?;
    Ok(())
}

pub async fn find_by_contract(
    contract: impl Into<String>,
    blockchain: Blockchain,
) -> anyhow::Result<Vec<(GeckoChain::Model, Vec<GeckoList::Model>)>> {
    let blockchain = blockchain.to_gecko();
    let contract = contract.into();

    let data = GeckoChain::Entity::find()
        .filter(GeckoChain::Column::Contract.eq(contract))
        .filter(GeckoChain::Column::Blockchain.eq(blockchain))
        .find_with_related(GeckoList::Entity)
        .all(DB_GECKO::get_connection().await)
        .await?;

    Ok(data)
}

pub async fn find_by_name(name: String) -> anyhow::Result<GeckoList::Model> {
    let data = GeckoList::Entity::find()
        .filter(GeckoList::Column::GeckoId.eq(name))
        .one(DB_GECKO::get_connection().await)
        .await?
        .unwrap();

    Ok(data)
}

pub async fn find_by_blockchain(
    blockchain: Blockchain,
) -> anyhow::Result<Vec<(GeckoChain::Model, Vec<GeckoList::Model>)>> {
    let blockchain = blockchain.to_gecko();

    let data = GeckoChain::Entity::find()
        .filter(GeckoChain::Column::Blockchain.eq(blockchain))
        .find_with_related(GeckoList::Entity)
        .limit(100)
        .all(DB_GECKO::get_connection().await)
        .await?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use crate::blockchain::Blockchain;
    use crate::db;

    #[tokio::test]
    async fn push_all_test() {
        db::coingecko::coingecko::update_all_gecko_list()
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn fill_gecko() {
        db::coingecko::coingecko::gecko_list_to_gecko_chain()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn find_by_contract() {
        db::coingecko::coingecko::find_by_contract(
            "0xb26c4b3ca601136daf98593feaeff9e0ca702a8d",
            Blockchain::Eth,
        )
        .await
        .unwrap();
    }
}
