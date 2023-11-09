use crate::{Context, Error};

#[poise::command(track_edits, slash_command)]
pub async fn gimmerole(ctx: Context<'_>, role_a: String) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap(); 
    if !role_a.to_lowercase().contains("ctf") && !role_a.to_lowercase().contains("finals") {
        ctx.send(|m| m.content("Nope").ephemeral(true)).await?;
        return Ok(());
    }
    if role_a == "ring0" || role_a == "Bot master" || role_a == "Channel master" {
        ctx.send(|m| m.content("Nope").ephemeral(true)).await?;
        return Ok(());
    }
    let roles = guild_id.roles(&ctx).await?;
    let role_b;
    if !roles.iter().any(|(_, role)| role.name == role_a) {
        ctx.send(|m| m.content("Role doesn't exist").ephemeral(true)).await?;
        return Ok(())
    } else {
        role_b = roles
            .iter()
            .find(|(_, role)| role.name == role_a)
            .unwrap()
            .1
            .clone();
    }
    let author = ctx.author();
    let mut member = guild_id.member(&ctx, author.id).await?;
    member.add_role(&ctx, role_b).await?;
    ctx.send(|m| m.content("Role added").ephemeral(true)).await?;
    Ok(())
}
