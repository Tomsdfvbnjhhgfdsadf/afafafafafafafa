use crate::CONFIG;
use ethers::prelude::{Address, LocalWallet, Signer};
use std::process::Stdio;
use std::str::FromStr;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::time::timeout;

const PROFANITY_FILE: &str = "./vangen.x64";
const TIMEOUT: Duration = Duration::from_secs(60 * 10);

#[derive(Debug)]
pub struct Wallet {
    pub private_key: LocalWallet,
    pub priv_key_string: String,
    pub address: Address,
}

pub async fn generate_wallet(address: Address) -> anyhow::Result<Wallet> {
    let (wallet, wallet_full) = profanity_wallet_string(address).await;
    loop {
        let data = profanity_generate(wallet.clone(), wallet_full.clone()).await?;
        if data.address == Address::from_str("0x0000000000000000000000000000000000000000").unwrap()
        {
            continue;
        } else {
            return Ok(data);
        }
    }
}

/* pub async fn generate_wallet(address: Address) -> Wallet {
    let (wallet, _start, _end) = wallet_string(address).await;
    loop {
        let data = generate_profanity(wallet.clone()).await;
        if data.address == Address::from_str("0x0000000000000000000000000000000000000000").unwrap()
        {
            continue;
        } else {
            return data;
        }
    }
} */

#[allow(dead_code)]
async fn wallet_string(address: Address) -> (String, String, String) {
    let wallet = &hex::encode(address);

    let start = &wallet[0..CONFIG.start_index];
    let end = &wallet[wallet.len() - CONFIG.end_index..];

    let result = format!("^0x{}(.*.){}$", start, end);
    (result, start.to_string(), end.to_string())
}

#[allow(dead_code)]
async fn generate_profanity(regex: String) -> Wallet {
    let mut command = Command::new(PROFANITY_FILE)
        .arg("-C")
        .arg("ETH")
        .arg("-r")
        .arg(regex)
        .current_dir("vanitygen-plusplus")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let mut res = Wallet {
        private_key: (&CONFIG.polygon_default_wallet).clone(),
        priv_key_string: "".to_string(),
        address: Default::default(),
    };

    let data = command.stdout.take().unwrap();
    let mut reader = BufReader::new(data).lines();
    while let Some(line) = reader.next_line().await.unwrap() {
        if line.contains("ETH Privkey: ") {
            let private_key_start = line.find("ETH Privkey: ").unwrap() + "ETH Privkey: ".len();
            let private_key_string = &line[private_key_start..private_key_start + 66];

            let private_key =
                LocalWallet::from_str(&*private_key_string.replace("0x", "")).unwrap();

            res.priv_key_string = private_key_string.to_string();
            res.address = private_key.address();
            res.private_key = private_key;
        } else {
            continue;
        }
    }
    command.kill().await.unwrap();
    res
}

async fn profanity_wallet_string(address: Address) -> (String, String) {
    let wallet = &hex::encode(address);
    let wallet = wallet.replace("0x", "");

    let start = &wallet[0..CONFIG.start_index];
    let end = &wallet[wallet.len() - CONFIG.end_index..];

    let result = format!("{}.{}", start, end);
    (result, wallet.clone())
}

#[allow(unused_variables)]
async fn profanity_generate(
    wallet: impl Into<String>,
    full_wallet: String,
) -> anyhow::Result<Wallet> {
    let mut command = Command::new(PROFANITY_FILE)
        .arg(wallet.into())
        .current_dir("vanitygen-plusplus")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let mut res = Wallet {
        private_key: (&CONFIG.polygon_default_wallet).clone(),
        priv_key_string: "".to_string(),
        address: Default::default(),
    };

    let data = command.stdout.take().unwrap();
    let mut reader = BufReader::new(data).lines();

    let timeout_result = timeout(TIMEOUT, async {
        while let Some(line) = reader.next_line().await.unwrap() {
            if line.contains("Private: ") {
                let private_key_start = line.find("Private: ").unwrap() + "Private: ".len();
                let private_key_string = &line[private_key_start..private_key_start + 66];

                let private_key =
                    LocalWallet::from_str(&*private_key_string.replace("0x", "")).unwrap();

                res.priv_key_string = private_key_string.to_string();
                res.address = private_key.address();
                res.private_key = private_key;

                break;
            }
        }
    })
    .await;

    match timeout_result {
        Ok(_) => {
            command.kill().await.unwrap();
            Ok(res)
        }
        Err(_) => {
            command.kill().await.unwrap();
            anyhow::bail!("10 minuts timeout")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wallet::{generate_wallet, profanity_wallet_string, wallet_string};
    use ethers::prelude::Address;
    use std::str::FromStr;

    #[tokio::test()]
    async fn generate_test() {
        let data = profanity_wallet_string(
            Address::from_str("0x1a71601567247199e7cf74fc6bf67e0ce6120380").unwrap(),
        )
        .await;
        // assert_eq!(data, "1aXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX20380".to_string());
    }
}
