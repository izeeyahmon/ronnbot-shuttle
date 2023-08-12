use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub pairs: Vec<Pair>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pair {
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "dexId")]
    // pub dex_id: String,
    pub url: String,
    #[serde(rename = "pairAddress")]
    pub pair_address: String,
    // #[serde(default)]
    // pub labels: Vec<String>,
    #[serde(rename = "baseToken")]
    pub base_token: BaseToken,
    #[serde(rename = "quoteToken")]
    pub quote_token: QuoteToken,
    #[serde(rename = "priceNative")]
    pub price_native: String,
    #[serde(rename = "priceUsd")]
    pub price_usd: String,
    // pub txns: Txns,
    pub volume: Volume,
    #[serde(rename = "priceChange")]
    pub price_change: PriceChange,
    pub liquidity: Liquidity,
    // #[serde(rename = "pairCreatedAt")]
    // pub pair_created_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseToken {
    pub address: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteToken {
    pub address: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    pub h24: f64,
    pub h6: f64,
    pub h1: f64,
    pub m5: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceChange {
    pub h24: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Liquidity {
    pub usd: f64,
    pub base: f64,
    pub quote: f64,
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("coin")
        .description("Get the CoinDetails from DexScreener")
        .create_option(|option| {
            option
                .name("coinname")
                .description("String Name of the Collection")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

pub async fn run(options: &[CommandDataOption]) -> Result<Root, anyhow::Error> {
    let option = options
        .get(0)
        .expect("Expected Query")
        .resolved
        .as_ref()
        .expect("Query");

    let apiresult = reqwest::get(format!(
        "https://api.dexscreener.com/latest/dex/search?q={:?}",
        option
    ))
    .await
    .unwrap()
    .json::<Root>()
    .await;
    match apiresult {
        Ok(ap) => Ok(ap),
        Err(_) => Err(anyhow!("Error Parsing Json")),
    }
}
