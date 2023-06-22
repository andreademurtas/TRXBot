use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let help: &str = "These are the commands you can use:

    `/help` - display this help message
    ";
    msg.channel_id.say(&ctx.http, help).await?;
    Ok(())
}
