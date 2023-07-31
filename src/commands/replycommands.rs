use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn izee(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "My Master aint no Simp!")
        .await?;

    Ok(())
}
#[command]
async fn josh(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.content(
                "<@263925875374948352>https://gfycat.com/needyunequaledattwatersprairiechicken",
            );

            m
        })
        .await?;
    Ok(())
}

#[command]
async fn panels(ctx: &Context, msg: &Message) -> CommandResult {
    let f = [(
        &tokio::fs::File::open("src/images/panels.jpg").await?,
        "panels.jpg",
    )];
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.files(f);

            m
        })
        .await?;

    Ok(())
}

#[command]
async fn swypes(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "https://live.staticflickr.com/1320/1440875785_f1619cdd1f.jpg",
        )
        .await?;

    Ok(())
}

#[command]
async fn zyo(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "https://i.chzbgr.com/original/7870277376/h6CACF993/beer-pok%C3%A9mon-wtf-pikachu-funny-7870277376")
        .await?;

    Ok(())
}

#[command]
async fn ziz(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("ziz crush")
                    .description("gay malaysian architect he looks even better with no clothes")
                    .timestamp(Timestamp::now())
                    .image("attachment://zizan.png")
            })
            .add_file("src/images/zizan.png")
        })
        .await?;

    Ok(())
}

#[command]
async fn flipcreed(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "https://cdn.discordapp.com/attachments/949923131257131091/960481942783008768/unknown.png")
        .await?;

    Ok(())
}

#[command]
async fn absinthe(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "https://media.discordapp.net/attachments/909469208176369716/956391913055985734/unknown.png")
        .await?;

    Ok(())
}

#[command]
async fn zilbag(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "https://i.imgur.com/2xNmKWi.png")
        .await?;

    Ok(())
}
#[command]
async fn ilv(ctx: &Context, msg: &Message) -> CommandResult {
    let text = "Exactly, buying ILV to me isn't like buying some meme coin for \"potential\" metaverse projects like GALA/ATLAS. There are so many talented people behind the Illuvium brand that buying and staking ILV is like buying shares in Apple when it was a baby company. I've been an avid gamer for many years, and I can tell there is tremendous talent on the Illuvium team. When I buy ILV, I'm not just buying some token. I'm buying early access to all the profits that the ILV team generate over the years.  If Illuvium products (games, tv shows, movies, merch, NFT) takes off because of the tremendous projects the ILV team do, then that's how Illuvium becomes the \"Apple\" of crypto gaming. I believe the team has the talent to pull it off. So keep up the good work on the art and the game @Grant | Illuvium @Andrew | Illuvium! ILV will be the blue chip token of the crypto gaming center that shows everyone how it's done. I think Illuvium is going to be that project after many hours of research into various metaverse projects. But we'll see how things play out in the long term. I'm still very optimistic on ILV even if my token is temporarily at a loss. The markets could get worse coming up here, but that just means I can buy more ILV at better prices. It's like buying Apple after their massive dip when hype died down and Steve Jobs wasn't there for many years. ILV is such a bargain at this price in my opinion, because the ILV team has got what it takes to make a A+ tier final product.";

    msg.channel_id.say(&ctx.http, text).await?;

    Ok(())
}

#[command]
async fn gm(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "GM <a:sykablyatdance:956543246560002108>")
        .await?;

    Ok(())
}

#[command]
async fn gn(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "GN <a:gotocave:956627835076902981>")
        .await?;

    Ok(())
}

#[command]
async fn fraggy(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Fraggy Rant")
                    .timestamp(Timestamp::now())
                    .image("attachment://fraggy.jpg")
            })
            .add_file("src/images/fraggy.jpg")
        })
        .await?;

    Ok(())
}
