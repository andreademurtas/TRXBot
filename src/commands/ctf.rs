use crate::{Context, Error};

#[poise::command(track_edits, slash_command)]
pub async fn list_participants(ctx: Context<'_>, ctf: String) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let roles = guild_id.roles(&ctx).await?;
    let role = roles
        .iter()
        .find(|(_, role)| role.name == ctf)
        .unwrap()
        .1
        .clone();
    let members = guild_id.members(&ctx, None, None).await?;
    let mut participants = String::new();
    for member in members {
        if member.roles.contains(&role.id) {
            participants.push_str(&member.user.name);
            participants.push('\n');
        }
    }
    ctx.send(|m| m.content(participants)).await?;
    Ok(())
}
