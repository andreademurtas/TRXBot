use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::channel::{PermissionOverwrite, PermissionOverwriteType};
use serenity::model::Permissions;
use serenity::utils::Colour;


use crate::ShardManagerContainer;

#[command]
#[allowed_roles("Bot master")]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.reply(ctx, "Shutting down!").await?;
        manager.lock().await.shutdown_all().await;
    }
    else {
        msg.reply(ctx, "There was a problem getting the shard manager").await?;
        return Ok(());
    }
    Ok(())
}

#[command]
#[allowed_roles("Bot master")]
async fn new(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if let Some(ctf) = args.single::<String>().ok() {
        if let Some(guild_id) = msg.guild_id {
            let category = guild_id.create_channel(&ctx.http, |c| c.name(&ctf).kind(ChannelType::Category)).await?;
            let general_public = guild_id.create_channel(&ctx.http, |c| c.name("general-public").kind(ChannelType::Text).category(category.id)).await?;
            let guild = guild_id.to_partial_guild(&ctx.http).await?;
            let rgb = Colour::from_rgb(rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>());
            let role = guild.create_role(&ctx.http, |r| r.name(&ctf).colour(rgb.0 as u64)).await?;
            let role_permission = PermissionOverwrite {
                allow: Permissions::all(),
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Role(role.id),
            };
            let everyone_permission = PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::all(),
                kind: PermissionOverwriteType::Role(RoleId::from(guild.id.0)),
            };
            let general = guild_id.create_channel(&ctx.http, |c| c.name("general").kind(ChannelType::Text).category(category.id)).await?;
            general.create_permission(&ctx.http, &everyone_permission).await?;
            general.create_permission(&ctx.http, &role_permission).await?;
            let category_permissions = PermissionOverwrite {
                allow: Permissions::all(),
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Role(role.id),
            };
            category.create_permission(&ctx.http, &category_permissions).await?;
            let message = general_public.id.send_message(&ctx.http, |m| {
                m.content(format!("Click the button to play {}", ctf))
                    .components(|c| 
                                c.create_action_row(|r| r.create_button(|b| b
                                                                        .style(ButtonStyle::Primary)
                                                                        .label("Play")
                                                                        .custom_id(String::from("play_") + &ctf))))
            }).await?;
            let interaction = match message.await_component_interaction(&ctx).await {
                Some(x) => x,
                None => {
                    msg.reply(&ctx, "Something went wrong").await?;
                    return Ok(());
                },
            };
            let member = interaction.member.as_ref().unwrap();
            let mut roles = member.roles.clone();
            roles.push(role.id);
            member.edit(&ctx.http, |m| m.roles(roles)).await?;
            interaction.create_interaction_response(&ctx.http, |r| r.kind(InteractionResponseType::DeferredUpdateMessage)).await?;
        }
    }
    else {
        msg.reply(ctx, "There was a problem getting the shard manager").await?;
        return Ok(());
    }
    Ok(())
}
