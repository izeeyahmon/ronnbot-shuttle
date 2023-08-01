use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn reactionroles(ctx: &Context, msg: &Message) -> CommandResult {
    let message =msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Reaction Roles")
                    .description(
                        "<a:gib:956543324410507284> for giveaways given by Collab People
                        <a:fraggy_spit:1084701921392218172> for Burning away Money to Zil
                        :parrot: for PirateNationBrick Announcement By zyo
                        <:pepefingerping:956560593819693087> or Any other Pings that Fuckers want to ping
                        :coin: for Alt-coins Buy opps?",
                    )
                    .timestamp(Timestamp::now())
            })
        })
        .await?;

    message
        .react(
            &ctx.http,
            ReactionType::Custom {
                animated: (true),
                id: (EmojiId(956543324410507284)),
                name: (Some(String::from("gib"))),
            },
        )
        .await?;

    message
        .react(
            &ctx.http,
            ReactionType::Custom {
                animated: (true),
                id: (EmojiId(1084701921392218172)),
                name: (Some(String::from("fraggy_spit"))),
            },
        )
        .await?;

    message
        .react(&ctx.http, ReactionType::Unicode("🦜".to_string()))
        .await?;

    message
        .react(
            &ctx.http,
            ReactionType::Custom {
                animated: (false),
                id: (EmojiId(956560593819693087)),
                name: (Some(String::from("pepefingerping"))),
            },
        )
        .await?;

    message
        .react(&ctx.http, ReactionType::Unicode("🪙".to_string()))
        .await?;
    Ok(())
}
