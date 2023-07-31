use anyhow::{anyhow, Result};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use std::env;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub collections: Vec<Collection>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub name: String,
    pub floor_ask: FloorAsk,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FloorAsk {
    pub source_domain: String,
    pub price: Option<Price>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub amount: Amount,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    pub decimal: f64,
    pub usd: f64,
    pub native: f64,
}
pub async fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected Collection Name")
        .resolved
        .as_ref()
        .expect("Expected Collection Object");
    let verbose_flag = match options.get(1) {
        Some(verb) => verb.value.as_ref().unwrap().as_bool().unwrap(),
        None => false,
    };

    if let CommandDataOptionValue::String(collection) = option {
        let api_result = call_api(collection).await;
        dbg!(&api_result);
        let mut aggregated_output = String::new();
        match api_result {
            Ok(api_output) => {
                let result_length = *&api_output.collections.len() as u32;
                if result_length == 0 {
                    return format!("There is no collection found for the name {} ", collection);
                }
                if verbose_flag {
                    for project in &api_output.collections {
                        let proj_name = &project.name;
                        //let floor_price = &project.floor_ask.price.clone().unwrap().amount.decimal;
                        let floor_price = match &project.floor_ask.price {
                            Some(p) => p.amount.decimal,
                            None => 0.0,
                        };
                        let floor_source = &project.floor_ask.source_domain;
                        let temp_string = format!(
                            "The floor price for [{}] is [{}]ETH and is on [{}]\n",
                            proj_name, floor_price, floor_source
                        );
                        aggregated_output.push_str(&temp_string);
                    }
                    aggregated_output
                } else {
                    let floor_price = match &api_output.collections[0].floor_ask.price {
                        Some(p) => p.amount.decimal,
                        None => 0.0,
                    };
                    let project_name = &api_output.collections[0].name;
                    let floor_source = &api_output.collections[0].floor_ask.source_domain;
                    format!(
                        "The floor price for [{}] is [{}]ETH and is on [{}]",
                        project_name, floor_price, floor_source
                    )
                }
            }
            Err(_) => "Something went wrong contact izee".to_string(),
        }
    } else {
        "Please Provide a collection name".to_string()
    }
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("floorprice")
        .description("Get a Floor Price of a Collection")
        .create_option(|option| {
            option
                .name("project")
                .description("String Name of the Collection")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("verbose")
                .description("If we pull display all the verbose stuff")
                .kind(CommandOptionType::Boolean)
                .required(false)
        })
}

pub async fn call_api(nft_collection: &String) -> Result<Root, anyhow::Error> {
    let url = format!(
        "https://api.reservoir.tools/collections/v6?name={}",
        nft_collection
    );
    dbg!(&url);

    let mut headers = HeaderMap::new();
    {
        let api_key = env::var("RESERVOIR_API_KEY").expect("NO API?");
        headers.insert(
            HeaderName::from_static("x-api-key"),
            HeaderValue::from_str(api_key.as_str()).unwrap(),
        );
    }

    headers.insert(
        HeaderName::from_static("accept"),
        HeaderValue::from_static("*/*"),
    );

    let response = reqwest::Client::new()
        .get(&url)
        .headers(headers)
        .send()
        .await
        .unwrap();
    dbg!(&response);
    match response.status() {
        StatusCode::OK => Ok(response.json::<Root>().await?),
        other => Err(anyhow!("Error Contacting API {}", other)),
    }
}
