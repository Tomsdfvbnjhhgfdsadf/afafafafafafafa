use crate::blockchain::Blockchain;
use anyhow::anyhow;
use async_once::AsyncOnce;
use ethers::prelude::{Address, Signer};
use ethers::signers::LocalWallet;
use lazy_static::lazy_static;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, ConnectOptions, Database, IntoActiveModel, SelectModel};
use std::fs;
use std::path::PathBuf;

lazy_static! {
    pub static ref DB_PENDING: AsyncOnce<DatabaseConnection> =
        AsyncOnce::new(DB_PENDING::create_db_connection());
    pub static ref PATH_PENDING: String = {
        let full_path = fs::canonicalize(PathBuf::from("src/db/pending.sqlite")).unwrap();
        let path = full_path.to_str().unwrap();
        format!("sqlite:{}", path)
    };
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pending")]
pub struct Model {
    pub address: String,
    pub private_key: String,
    pub send_to: String,
    pub network: String,
    #[sea_orm(primary_key, auto_increment = true)]
    pub rowid: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl DB_PENDING {
    async fn get_connection() -> &'static DatabaseConnection {
        DB_PENDING.get().await
    }

    async fn create_db_connection() -> DatabaseConnection {
        let mut opt = ConnectOptions::new((&*PATH_PENDING).clone());
        opt.max_connections(500).sqlx_logging(false);
        // .sqlx_logging_level(log::LevelFilter::Error);

        let db = Database::connect(opt)
            .await
            .expect("Database connection pool creation failed");

        db
    }
}

pub async fn push_to_db(
    send_to: Address,
    wallet: &LocalWallet,
    priv_key: impl Into<String>,
    blockchain: Blockchain,
) -> anyhow::Result<()> {
    let mut model = Model {
        address: "0x".to_owned() + &hex::encode(wallet.address()),
        private_key: priv_key.into(),
        send_to: "0x".to_owned() + &hex::encode(send_to),
        network: String::from(blockchain),
        rowid: 0,
    }
    .into_active_model();
    model.rowid = ActiveValue::NotSet;

    model.insert(DB_PENDING::get_connection().await).await?;
    Ok(())
}

pub async fn db_get_all() -> anyhow::Result<Vec<Model>> {
    let data = Entity::find()
        .all(DB_PENDING::get_connection().await)
        .await?;
    Ok(data)
}

pub async fn db_delete(wallet: &LocalWallet) -> anyhow::Result<()> {
    let data = Entity::find()
        .filter(Column::Address.contains(&*("0x".to_owned() + &hex::encode(wallet.address()))))
        .one(DB_PENDING::get_connection().await)
        .await?;

    if let Some(value) = data {
        value.delete(DB_PENDING::get_connection().await).await?
    } else {
        return Err(anyhow!("err"));
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::blockchain::Blockchain;
    use crate::db::pending::{db_get_all, DB_PENDING};
    use crate::CONFIG;
    use ethers::abi::Address;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::str::FromStr;

    #[tokio::test()]
    async fn test_db_connection() {
        DB_PENDING::get_connection().await;
    }

    #[tokio::test()]
    async fn test_get_all() {
        let b = db_get_all().await.unwrap();
        println!("{:?}", b);
    }
}
