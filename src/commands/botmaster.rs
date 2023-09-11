use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::model::application::component::ButtonStyle;
use serenity::model::channel::ChannelType;
use serenity::model::channel::{PermissionOverwrite, PermissionOverwriteType};
use serenity::model::prelude::RoleId;
use serenity::model::Permissions;
use serenity::utils::Colour;
use teloxide::prelude::*;

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
pub async fn reload(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap(); 
    let general_public = ctx
        .channel_id();
    match general_public.name(&ctx).await {
        Some(name) => {
            if name != "general-public" {
                ctx.send(|b| b.content("This command must be run in the general-public channel").ephemeral(true)).await?;
                return Ok(())
            }
        }
        None => {
            ctx.send(|b| b.content("This command must be run in the general-public channel").ephemeral(true)).await?;
            return Ok(())
        }
    }
    let message_result = general_public
        .messages(&ctx, |m| m.limit(100000))
        .await?;
    println!("{:?}", message_result);
    let message = message_result
        .iter()
        .find(|m| m.content.contains("Click the button to play"))
        .unwrap();
    let mut words = message.content.split_whitespace();
    let ctf = words.next_back().unwrap();
    let roles = guild_id.roles(&ctx).await?;
    let role;
        if !roles.iter().any(|(_, role)| role.name == ctf) {
            ctx.send(|b| b.content("Ctf doesn't exist").ephemeral(true)).await?;
            return Ok(())
        } else {
            role = roles
                .iter()
                .find(|(_, role)| role.name == ctf)
                .unwrap()
                .1
                .clone();
        }
    message.delete(&ctx).await?;
    general_public
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
pub async fn discord_telegram_message(ctx: Context<'_>, msg: String) -> Result<(), Error> {
    ctx.send(|c| {
        c.content(msg.clone())
    })
        .await?;
    let bot = Bot::from_env();
    let chat_id = std::env::var("TELEGRAM_CHAT_ID").unwrap();
    let disclaimer = "```(This message was sent from Discord)```";
    bot.send_message(chat_id, format!("{}\n{}", disclaimer, msg)).await?;
    bot.close().await?;
    Ok(())
}
