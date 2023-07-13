#![warn(clippy::str_to_string)]

mod commands;

use dotenv::dotenv;
use log::{debug, error, info, log_enabled, Level};
use poise::serenity_prelude as serenity;
use std::{collections::HashMap, env, sync::Mutex, time::Duration};
use serenity::async_trait;
use serenity::EventHandler;
use serenity::GuildChannel;
use serenity::model::channel::{PermissionOverwrite, PermissionOverwriteType};
use serenity::model::Permissions;
use serenity::model::prelude::RoleId;

use crate::commands::botmaster::*;
use crate::commands::ctftime::*;
use crate::commands::factordb::*;
use crate::commands::gg::*;
use crate::commands::help::*;
use crate::commands::tle::*;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn channel_create(&self, ctx: serenity::Context, channel: &GuildChannel) {
        let roles = channel.guild_id.roles(&ctx).await.unwrap();
        let category = channel.parent_id.unwrap();
        for (id, role) in roles {
            if role.name == category.name(&ctx).await.unwrap() {
                 let everyone_permission = PermissionOverwrite {
                    allow: Permissions::empty(),
                    deny: Permissions::all(),
                    kind: PermissionOverwriteType::Role(RoleId::from(channel.guild_id.0)),
                };

                let role_permission = PermissionOverwrite {
                    allow: Permissions::all(),
                    deny: Permissions::empty(),
                    kind: PermissionOverwriteType::Role(role.id),
                }; 

                channel.create_permission(&ctx, &everyone_permission).await.unwrap();
                channel.create_permission(&ctx, &role_permission).await.unwrap();
            }
        }
    }
}

pub struct Data {}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();

    let options = poise::FrameworkOptions {
        commands: vec![
            help(),
            gg(),
            shutdown(),
            new(),
            ctftime(),
            factordb(),
            moonlighter(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    };

    poise::Framework::builder()
        .token(
            env::var("DISCORD_TOKEN")
                .expect("Missing `DISCORD_TOKEN` env var, see README for more information."),
        )
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::all())
        .client_settings(|client| {
            client
                .event_handler(Handler)
        })
        .run()
        .await
        .unwrap();
}
