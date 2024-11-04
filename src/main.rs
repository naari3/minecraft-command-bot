use std::sync::{atomic::AtomicBool, Arc};

use config::Config;
use domains::send_rule::SendRule;
use dotenv::dotenv;
use error::Error;
use listeners::event_handler;
use log::{debug, info};
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};
use serenity::all::GatewayIntents;

mod commands;
mod config;
mod domains;
mod error;
mod listeners;
mod minecraft_line;

use commands::{builtins, cmd::*, mcuuid::*, say::*};

use crate::domains::send_rule::{
    advancement_rule::AdvancementRule, chat_rule::ChatRule, death_rule::DeathRule,
    login_rule::LoginRule, rcon_rule::RconRule, server_rule::ServerRule,
};

struct Data {
    send_rules: Arc<Vec<Box<dyn SendRule + Send + Sync>>>,
    is_loop_running: AtomicBool,
}

type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or(format!("minecraft_command_bot={}", log_level)),
    )
    .init();

    let config = Config::get();

    debug!("config: {:?}", config);
    info!("Start command bot");

    let options = FrameworkOptions {
        prefix_options: PrefixFrameworkOptions {
            prefix: Some(config.discord_bot_prefix.clone()),
            non_command_message: Some(|_, _, msg| {
                Box::pin(async move {
                    debug!("non command message: {}", msg.content);
                    Ok(())
                })
            }),
            ..Default::default()
        },
        commands: vec![
            cmd(),
            mcuuid(),
            say(),
            ping(),
            builtins::help(),
            builtins::pretty_help(),
            builtins::register(),
        ],
        on_error: |error| {
            Box::pin(async move {
                log::error!("Error: {:?}", error.to_string());
                poise::builtins::on_error(error).await.unwrap();
            })
        },
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };
    let framefork = Framework::builder()
        .options(options)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                info!("Registered commands");
                Ok(Data {
                    is_loop_running: AtomicBool::new(false),
                    send_rules: Arc::new(vec![
                        Box::new(ChatRule) as _,
                        Box::new(RconRule) as _,
                        Box::new(LoginRule) as _,
                        Box::new(AdvancementRule) as _,
                        Box::new(ServerRule) as _,
                        Box::new(DeathRule) as _,
                    ]),
                })
            })
        })
        .build();

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::all::ClientBuilder::new(&config.discord_bot_token, intents)
        .framework(framefork)
        .await
        .expect("クライアントの作成中にエラーが発生しました");

    if let Err(reason) = client.start().await {
        eprintln!("クライアントの起動に失敗しました: {:?}", reason);
    }
}
