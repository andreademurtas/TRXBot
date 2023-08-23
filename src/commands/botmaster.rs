use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::model::application::component::ButtonStyle;
use serenity::model::channel::ChannelType;
use serenity::model::channel::{PermissionOverwrite, PermissionOverwriteType};
use serenity::model::prelude::RoleId;
use serenity::model::Permissions;
use serenity::utils::Colour;

async fn check_botmaster(ctx: Context<'_>) -> Result<bool, Error> {
    let guild_id = ctx.guild_id().unwrap();
    let roles = guild_id.roles(&ctx).await?;
    for (id, role) in roles {
        if role.name == "Bot master" || role.name == "ring0" {
            return Ok(ctx.author().has_role(&ctx, guild_id, id).await?);
        }
    }
    Ok(false)
}

#[poise::command(track_edits, prefix_command, hide_in_help, check = "check_botmaster")]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    ctx.framework()
        .shard_manager()
        .lock()
        .await
        .shutdown_all()
        .await;
    Ok(())
}

#[poise::command(track_edits, slash_command, hide_in_help, check = "check_botmaster")]
pub async fn new(
    ctx: Context<'_>,
    #[description = "The name of the CTF"] ctf: String,
) -> Result<(), Error> {
    ctx.send(|b| b.content("ok").ephemeral(true)).await?;
    let guild_id = ctx.guild_id().unwrap();
    let rgb = Colour::from_rgb(
        rand::random::<u8>(),
        rand::random::<u8>(),
        rand::random::<u8>(),
    );
    let roles = guild_id.roles(&ctx).await?;
    let role;
    if !roles.iter().any(|(_, role)| role.name == ctf) {
        role = guild_id
            .create_role(&ctx, |r| r.name(&ctf).colour(rgb.0 as u64))
            .await?;
    } else {
        role = roles
            .iter()
            .find(|(_, role)| role.name == ctf)
            .unwrap()
            .1
            .clone();
    }
    let category = guild_id
        .create_channel(&ctx, |c| c.name(&ctf).kind(ChannelType::Category))
        .await?;
    let general_public = guild_id
        .create_channel(&ctx, |c| {
            c.name("general-public")
                .kind(ChannelType::Text)
                .category(category.id)
        })
        .await?;
    let general = guild_id
        .create_channel(&ctx, |c| {
            c.name("general")
                .kind(ChannelType::Text)
                .category(category.id)
        })
        .await?;
    let everyone_permission = PermissionOverwrite {
        allow: Permissions::empty(),
        deny: Permissions::all(),
        kind: PermissionOverwriteType::Role(RoleId::from(guild_id.0)),
    };
    let role_permission = PermissionOverwrite {
        allow: Permissions::all(),
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Role(role.id),
    };
    general
        .create_permission(&ctx, &everyone_permission)
        .await?;
    general.create_permission(&ctx, &role_permission).await?;
    category.create_permission(&ctx, &role_permission).await?;
    general_public
        .id
        .send_message(&ctx, |m| {
            m.content(format!("Click the button to play {}", ctf))
                .components(|c| {
                    c.create_action_row(|r| {
                        r.create_button(|b| {
                            b.style(ButtonStyle::Primary)
                                .label("Play")
                                .custom_id(String::from("play_") + &ctf)
                        })
                    })
                })
        })
        .await?;
    while let Some(interaction) = serenity::CollectComponentInteraction::new(ctx).await {
        let member = interaction.member.as_ref().unwrap();
        let mut roles = member.roles.clone();
        if !roles.contains(&role.id) {
            roles.push(role.id);
            member.edit(&ctx, |m| m.roles(roles)).await?;
        }
        interaction
            .create_interaction_response(ctx, |r| {
                r.kind(serenity::InteractionResponseType::DeferredUpdateMessage)
            })
            .await?;
    }
    Ok(())
}

#[poise::command(track_edits, slash_command, hide_in_help, check = "check_botmaster")]
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
