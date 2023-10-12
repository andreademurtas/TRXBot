use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(track_edits, slash_command)]
pub async fn gimmerole(ctx: Context<'_>, role: String) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap(); 
    if role == "ring0" || role == "Bot master" || role == "Channel master" {
        ctx.send(|m| m.content("Nope").ephemeral(true)).await?;
        return Ok(());
    }
    let roles = guild_id.roles(&ctx).await?;
    let role;
    if !roles.iter().any(|(_, role)| role.name == role) {
        ctx.send(|m| m.content("Role doesn't exist").ephemeral(true)).await?;
    } else {
        role = roles
            .iter()
            .find(|(_, role)| role.name == role)
            .unwrap()
            .1
            .clone();
    }
    let mut author = ctx.author();
    let mut member = guild_id.member(&ctx, author.id).await?;
    member.add_role(&ctx, role).await?;
    ctx.send(|m| m.content("Role added").ephemeral(true)).await?;
    Ok(())
}
