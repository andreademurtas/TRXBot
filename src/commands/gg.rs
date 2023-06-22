use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn gg(ctx: &Context, msg: &Message) -> CommandResult {
    let channel_id = msg.channel_id;
    let red_flag = "🚩";
    let name = channel_id.name(&ctx).await.unwrap();
    if name.contains(red_flag) {
        return Ok(());
    }
    channel_id.edit(&ctx.http, |c| {
        c.name(format!("{}{}", red_flag, name))
    }).await?;
    Ok(())
}
