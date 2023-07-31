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
                        "<:suhate:1077292647544279170> For Giveaways given by Collab People
                        <:ifiloseitall:1039730494226567168> for Burning away Money to Zil
                        <:izeewl:951674684431302699> for PirateNationBrick Announcement By zyo
                        <:richwhalefreed:1039730496290177096> or Any other Pings that Fuckers want to ping
                        <:ilikeboys:872842548350156830> for Alt-coins Buy opps?",
                    )
                    .timestamp(Timestamp::now())
            })
        })
        .await?;

    message
        .react(
            &ctx.http,
            ReactionType::Custom {
                animated: (false),
                id: (EmojiId(1077292647544279170)),
                name: (Some(String::from("suhate"))),
            },
        )
        .await?;

    message
        .react(
            &ctx.http,
            ReactionType::Custom {
                animated: (false),
                id: (EmojiId(1039730494226567168)),
                name: (Some(String::from("ifiloseitall"))),
            },
        )
        .await?;

    message
        .react(
            &ctx.http,
            ReactionType::Custom {
                animated: (false),
                id: (EmojiId(951674684431302699)),
                name: (Some(String::from("izeewl"))),
            },
        )
        .await?;

    message
        .react(
            &ctx.http,
            ReactionType::Custom {
                animated: (false),
                id: (EmojiId(1039730496290177096)),
                name: (Some(String::from("richwhalefreed"))),
            },
        )
        .await?;

    message
        .react(
            &ctx.http,
            ReactionType::Custom {
                animated: (false),
                id: (EmojiId(872842548350156830)),
                name: (Some(String::from("ilikeboys"))),
            },
        )
        .await?;
    Ok(())
}
