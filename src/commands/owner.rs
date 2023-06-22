use std::time::Duration;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::utils::Colour;


use crate::ShardManagerContainer;

#[command]
#[owners_only]
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
#[owners_only]
async fn new(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    if let Some(_manager) = data.get::<ShardManagerContainer>() {
        if let Some(ctf) = args.single::<String>().ok() {
            if let Some(guild_id) = msg.guild_id {
                let guild = guild_id.to_partial_guild(&ctx.http).await?;
                let rgb = Colour::from_rgb(rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>());
                let role = guild.create_role(&ctx.http, |r| r.name(&ctf).colour(rgb.0 as u64)).await?;
                let message = msg.channel_id.send_message(&ctx.http, |m| {
                    m.content(format!("Click the button to play {}", ctf))
                        .components(|c| 
                                    c.create_action_row(|r| r.create_button(|b| b
                                                                            .style(ButtonStyle::Primary)
                                                                            .label("Play")
                                                                            .custom_id(String::from("play_") + &ctf))))
                }).await?;
                let interaction = match message.await_component_interaction(&ctx).timeout(Duration::from_secs(60 * 3)).await {
                    Some(x) => x,
                    None => {
                        msg.reply(&ctx, "Timed out").await?;
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
    }
    else {
        msg.reply(ctx, "There was a problem getting the shard manager").await?;
        return Ok(());
    }
    Ok(())
}
