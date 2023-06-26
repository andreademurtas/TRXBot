use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn gg(ctx: Context<'_>) -> Result<(), Error> {
    let red_flag = "🚩";
    let name = ctx.channel_id().name(&ctx).await.unwrap();
    if name.contains(red_flag) {
        ctx.send(|c|{
            c.content("You don't solve a challenge twice, do you?")
                .reply(true)
            }).await?;
        return Ok(());
    }
    ctx.channel_id().edit(&ctx, |c| {
        c.name(format!("{}{}", red_flag, name))
    }).await?;
    ctx.send(|c|{
        c.content("gg")
            .reply(true)
        }).await?;
    Ok(())
}
