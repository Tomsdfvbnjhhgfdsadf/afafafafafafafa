use reqwest::header;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "_cache_seconds")]
    pub cache_seconds: i64,
    #[serde(rename = "_seconds")]
    pub seconds: f64,
    #[serde(rename = "_use_cache")]
    pub use_cache: bool,
    pub data: Data,
    #[serde(rename = "error_code")]
    pub error_code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub desc: Desc,
    #[serde(rename = "email_verified")]
    pub email_verified: bool,
    #[serde(rename = "email_verified_at")]
    pub email_verified_at: Value,
    #[serde(rename = "follower_count")]
    pub follower_count: i64,
    #[serde(rename = "following_count")]
    pub following_count: i64,
    pub id: String,
    #[serde(rename = "initial_price")]
    pub initial_price: i64,
    #[serde(rename = "is_followed")]
    pub is_followed: bool,
    #[serde(rename = "is_following")]
    pub is_following: bool,
    #[serde(rename = "is_pro")]
    pub is_pro: bool,
    #[serde(rename = "logo_thumbnail_url")]
    pub logo_thumbnail_url: String,
    #[serde(rename = "logo_url")]
    pub logo_url: String,
    #[serde(rename = "offer_price")]
    pub offer_price: i64,
    #[serde(rename = "replied_rate")]
    pub replied_rate: f64,
    pub tvf: f64,
    #[serde(rename = "twitter_id")]
    pub twitter_id: String,
    #[serde(rename = "twitter_id_verified")]
    pub twitter_id_verified: bool,
    #[serde(rename = "twitter_id_verified_at")]
    pub twitter_id_verified_at: Value,
    #[serde(rename = "uncharged_offer_count")]
    pub uncharged_offer_count: i64,
    #[serde(rename = "uncharged_offer_value")]
    pub uncharged_offer_value: i64,
    #[serde(rename = "unread_message_count")]
    pub unread_message_count: i64,
    #[serde(rename = "web3_id")]
    pub web3_id: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Desc {
    #[serde(rename = "born_at")]
    pub born_at: i64,
    pub cex: Cex,
    pub contract: Contract,
    pub id: String,
    #[serde(rename = "is_danger")]
    pub is_danger: Value,
    #[serde(rename = "is_spam")]
    pub is_spam: Value,
    pub name: Value,
    pub org: Org,
    pub protocol: Protocol,
    pub tags: Vec<Value>,
    #[serde(rename = "thirdparty_names")]
    pub thirdparty_names: ThirdpartyNames,
    #[serde(rename = "usd_value")]
    pub usd_value: f64,
    pub user: Option<User2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cex {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Org {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Protocol {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThirdpartyNames {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User2 {
    #[serde(rename = "logo_is_nft")]
    pub logo_is_nft: Option<bool>,
    #[serde(rename = "logo_thumbnail_url")]
    pub logo_thumbnail_url: Option<String>,
    #[serde(rename = "logo_url")]
    pub logo_url: Option<String>,
    pub memo: Option<Value>,
    #[serde(rename = "web3_id")]
    pub web3_id: Option<Value>,
}

pub async fn get_debank_banalce(
    wallet: impl Into<String>,
    proxy: Option<&str>,
) -> anyhow::Result<Root> {
    let wallet = wallet.into();
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "api.debank.com".parse().unwrap());
    headers.insert("accept", "*/*".parse().unwrap());
    headers.insert(
        "accept-language",
        "ru-RU,ru;q=0.9,en-US;q=0.8,en;q=0.7,uk;q=0.6"
            .parse()
            .unwrap(),
    );
    headers.insert("origin", "https://debank.com".parse().unwrap());
    headers.insert("referer", "https://debank.com/".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        "\"Not_A Brand\";v=\"99\", \"Google Chrome\";v=\"109\", \"Chromium\";v=\"109\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"macOS\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-site".parse().unwrap());
    headers.insert("sec-gpc", "1".parse().unwrap());
    headers.insert("source", "web".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36".parse().unwrap());

    let client = match proxy {
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
    };

    let res: Root = client
        .get(format!("https://api.debank.com/user?id={}", wallet))
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}
