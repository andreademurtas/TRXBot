use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::model::id::ChannelId;
use serenity::prelude::*;

#[command]
async fn gg(ctx: &Context, msg: &Message) -> CommandResult {
    let channel_id = msg.channel_id;
    let red_flag = "🚩";
    channel_id.edit(&ctx.http, |c| {
        c.name(format!("{} {}", red_flag, channel_id.name()))
    }).await?;
    Ok(())
}
