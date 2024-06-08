use crate::db;
use async_once::AsyncOnce;
use ethers::prelude::{Address, Signer};
use ethers::signers::LocalWallet;
use lazy_static::lazy_static;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveValue, ConnectOptions, Database, IntoActiveModel, SelectModel};
use std::fs;
use std::path::PathBuf;

lazy_static! {
    pub static ref DB_BALANCES: AsyncOnce<DatabaseConnection> =
        AsyncOnce::new(DB_BALANCES::create_db_connection());
    pub static ref PATH_BALANCES: String = {
        let full_path = fs::canonicalize(PathBuf::from("src/db/balances.sqlite")).unwrap();
        let path = full_path.to_str().unwrap();
        format!("sqlite:{}", path)
    };
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "balances")]
pub struct Model {
    pub address: String,
    pub private_key: String,
    pub send_to: String,
    pub network: String,
    pub balance: f64,
    #[sea_orm(primary_key, auto_increment = true)]
    pub rowid: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl DB_BALANCES {
    async fn get_connection() -> &'static DatabaseConnection {
        DB_BALANCES.get().await
    }

    async fn create_db_connection() -> DatabaseConnection {
        let mut opt = ConnectOptions::new((&*PATH_BALANCES).clone());
        opt.max_connections(500).sqlx_logging(false);
        // .sqlx_logging_level(log::LevelFilter::Error);

        let db = Database::connect(opt)
            .await
            .expect("Database connection pool creation failed");

        db
    }
}

pub async fn push_or_change(balance_usd: f64, wallet: db::Model) -> anyhow::Result<()> {
    if let Some(find) = find(wallet.address.clone()).await? {
        let mut active_model = find.into_active_model();
        active_model.balance = Set(balance_usd);
        active_model
            .update(DB_BALANCES::get_connection().await)
            .await?;
    } else {
        push(balance_usd, wallet).await?
    }
    Ok(())
}

async fn push(balance_usd: f64, wallet: db::Model) -> anyhow::Result<()> {
    let mut model = Model {
        address: wallet.address,
        private_key: wallet.private_key,
        send_to: wallet.send_to,
        network: wallet.network,
        balance: balance_usd,
        rowid: 0,
    }
    .into_active_model();
    model.rowid = ActiveValue::NotSet;

    model.insert(DB_BALANCES::get_connection().await).await?;
    Ok(())
}

async fn find(wallet: String) -> anyhow::Result<Option<Model>> {
    let find = Entity::find()
        .filter(Column::Address.contains(&*wallet))
        .one(DB_BALANCES::get_connection().await)
        .await?;
    Ok(find)
}
