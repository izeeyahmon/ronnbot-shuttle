use anyhow::anyhow;
use data::botmap::BotMap;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::Colour;
use shuttle_secrets::SecretStore;
use std::path::PathBuf;
use tracing::info;
mod commands;
mod data;
mod slashcommands;
use crate::commands::floor::*;
use crate::commands::meta::*;
use crate::commands::reactionroles::*;
use crate::commands::replycommands::*;
use crate::data::{config::Config, messagemap::MessageMap, reactionmap::ReactionMap};
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::{command, group, hook};
use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::event::ResumedEvent;
use serenity::model::prelude::ChannelId;
use serenity::model::{
    channel::{Reaction, ReactionType},
    id::RoleId,
};
use serenity::utils;
use serenity::utils::parse_emoji;
use std::collections::HashSet;
use std::io::copy;
use std::io::Cursor;
use std::sync::Arc;
use std::{
    convert::TryFrom,
    sync::atomic::{AtomicU64, Ordering},
};
pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Bot;
#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);
            let _ = command.defer(&ctx.http).await;
            let content = match command.data.name.as_str() {
                "floorprice" => {
                    let api_result = slashcommands::floorprice::run(&command.data.options).await;

                    command
                        .edit_original_interaction_response(&ctx.http, |response| {
                            response.content(api_result)
                        })
                        .await
                        .unwrap()
                }
                "coin" => {
                    if let Ok(api_result) = slashcommands::coin::run(&command.data.options).await {
                        dbg!(api_result.pairs[0].clone());
                        command
                            .create_followup_message(&ctx.http, |response| {
                                response.embed(|e| {
                                    e.title(api_result.pairs[0].to_owned().base_token.name)
                                        .url(api_result.pairs[0].clone().url)
                                        .field(
                                            "Price",
                                            format!(
                                                "${} : {}%",
                                                api_result.pairs[0].to_owned().price_usd.unwrap_or(
                                                    "No value you got rugged bruh".to_string()
                                                ),
                                                api_result.pairs[0].price_change.h24
                                            ),
                                            true,
                                        )
                                        .field(
                                            "Liquidity",
                                            format!(
                                                "${}",
                                                api_result.pairs[0]
                                                    .clone()
                                                    .liquidity
                                                    .unwrap_or_default()
                                                    .usd
                                            ),
                                            false,
                                        )
                                        .field(
                                            "Chain",
                                            format!(
                                                "${}@{}",
                                                api_result.pairs[0].clone().chain_id,
                                                api_result.pairs[0].clone().dex_id
                                            ),
                                            false,
                                        )
                                        .field(
                                            "VOL",
                                            format!("${}", api_result.pairs[0].clone().volume.h24,),
                                            false,
                                        )
                                        .colour(if api_result.pairs[0].price_change.h24 > 0.0 {
                                            Colour::DARK_GREEN
                                        } else {
                                            Colour::DARK_RED
                                        })
                                })
                            })
                            .await
                            .unwrap()
                    } else {
                        command
                            .create_followup_message(&ctx.http, |response| {
                                response.content("Some Error occured Contact Izeeyahmon.eth")
                            })
                            .await
                            .unwrap()
                    }
                }
                _ => command
                    .edit_original_interaction_response(&ctx.http, |response| {
                        response.content("Test")
                    })
                    .await
                    .unwrap(),
            };

            if let Err(why) = command
                .edit_original_interaction_response(&ctx.http, |response| {
                    response.content(content.content)
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
        let command = Command::create_global_application_command(&ctx.http, |command| {
            slashcommands::floorprice::register(command)
        })
        .await;

        println!(
            "I created the following global slash command: {:#?}",
            command
        );
        let command = Command::create_global_application_command(&ctx.http, |command| {
            slashcommands::coin::register(command)
        })
        .await;

        println!(
            "I created the following global slash command: {:#?}",
            command
        );
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        handle_reaction(ctx, reaction, true).await;
    }
    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        handle_reaction(ctx, reaction, false).await;
    }
}
async fn handle_reaction(ctx: Context, reaction: Reaction, add_role: bool) {
    let data_read = ctx.data.read().await;
    let message_data = data_read
        .get::<MessageMap>()
        .expect("Expected MessageMap in TypeMap.")
        .clone();
    if reaction.channel_id != ChannelId(message_data.load(Ordering::SeqCst)) {
        return;
    }
    if let Some(user) = reaction.user_id {
        if let Ok(struct_user) = user.to_user(&ctx.http).await {
            if struct_user.bot {
                return;
            }
        }
    }

    let reaction_roles_data = data_read
        .get::<ReactionMap>()
        .expect("Expected ReactionMap in TypeMap.")
        .clone();

    let reaction_roles = &*reaction_roles_data.read().await;
    for (emoji, role_id) in reaction_roles {
        if emoji != &reaction.emoji {
            continue;
        }

        if let Some(guild_id) = reaction.guild_id {
            if let Some(user_id) = reaction.user_id {
                if let Ok(mut member) = guild_id.member(&ctx, user_id).await {
                    if add_role {
                        if let Err(err) = member.add_role(&ctx, role_id).await {
                            println!("Role could not be added: {}", err);
                        }
                        println!(
                            "Role {} added to user {} by reacting with {}.",
                            role_id, member, emoji
                        )
                    } else {
                        if let Err(err) = member.remove_role(&ctx, role_id).await {
                            println!("Role could not be removed: {}", err);
                        }
                        println!(
                            "Role {} removed from user {} by un-reacting with {}.",
                            role_id, member, emoji
                        )
                    }
                }
            }
        }
    }
}

#[command]
async fn steal(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.message().is_empty() {
        msg.channel_id
            .say(&ctx.http, "Please supply some Emojis")
            .await?;
    } else if let Some(guild_id) = msg.guild_id {
        for emojis in args.message().split_whitespace() {
            let emoji = parse_emoji(emojis).unwrap();
            let image_url = emoji.url();
            let mut file = std::fs::File::create(&emoji.name).unwrap();
            let response = reqwest::get(&image_url).await?;
            let mut content = Cursor::new(response.bytes().await?);
            copy(&mut content, &mut file)?;
            let image = utils::read_image(&emoji.name).expect("Failed to read image");
            guild_id.create_emoji(&ctx, &emoji.name, &image).await?;
            std::fs::remove_file(&emoji.name).unwrap();
            msg.channel_id
                .say(&ctx.http, format!("I have added the emoji {}", &emoji.name))
                .await?;
        }
    }
    Ok(())
}
#[group]
#[commands(
    ping,
    izee,
    josh,
    swypes,
    zyo,
    ziz,
    flipcreed,
    absinthe,
    zilbag,
    ilv,
    gm,
    gn,
    panels,
    reactionroles,
    floor,
    fraggy,
    steal
)]

struct General;

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => println!("Processed command '{}'", command_name),
        Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_static_folder::StaticFolder(folder = "images")] images_folder: PathBuf,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let reservoir_key = if let Some(reservoir_key) = secret_store.get("RESERVOIR_API_KEY") {
        reservoir_key
    } else {
        return Err(anyhow!("Reservoir keyy was not found").into());
    };
    std::env::set_var("RESERVOIR_API_KEY", reservoir_key);
    let http = Http::new(&token);

    let config_2 = r#"{
    "channel_id": 1027793756872769536,
    "emotes": [
        "<a:gib:956543324410507284>",
        "<a:fraggy_spit:1084701921392218172>",
        "🦜",
        "<:pepefingerping:956560593819693087>",
        "🪙"
    ],
    "role_ids": [
        1027794564133691453, 968428801728069662, 1041328459877724250,
        1055667323534573709, 1088778873883336755
    ]
 }"#;
    let config: Config = serde_json::from_str(config_2).unwrap();

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!"))
        .group(&GENERAL_GROUP)
        .unrecognised_command(unknown_command)
        .after(after);
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_EMOJIS_AND_STICKERS;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .framework(framework)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        let mut reaction_roles = vec![];

        dbg!(for index in 0..config.emotes.len() {
            reaction_roles.push((
                ReactionType::try_from(config.emotes[index].as_str()).unwrap(),
                RoleId(config.role_ids[index]),
            ));
        });
        data.insert::<MessageMap>(Arc::new(AtomicU64::new(config.channel_id)));
        data.insert::<ReactionMap>(Arc::new(RwLock::new(reaction_roles)));
        data.insert::<BotMap>(images_folder);
    }
    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });
    Ok(client.into())
}
