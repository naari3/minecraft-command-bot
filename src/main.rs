use std::sync::Arc;

use config::Config;
use dotenv::dotenv;
use log::info;
use serenity::{
    all::{standard::Configuration, GatewayIntents},
    framework::{standard::macros::group, StandardFramework},
    Client,
};

mod commands;
mod config;
mod domains;
mod error;
mod globals;
mod listeners;
mod minecraft_line;

use commands::{cmd::*, help::*, mcuuid::*, say::*};

use crate::{
    domains::send_rule::{
        advancement_rule::AdvancementRule, chat_rule::ChatRule, death_rule::DeathRule,
        login_rule::LoginRule, rcon_rule::RconRule, server_rule::ServerRule,
    },
    globals::SendRules,
    listeners::{after_commands, dispatch_error, Handler},
};

#[group]
#[description("コマンドコマンド")]
#[summary("main")]
#[only_in(guilds)]
#[commands(cmd, mcuuid, say)]
struct General;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "debug".to_string());
    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or(format!("minecraft_command_bot={}", log_level)),
    )
    .init();

    let config = Config::get();

    info!("Start command bot");

    let framework = StandardFramework::new()
        // .before(before_commands)
        .after(after_commands)
        .on_dispatch_error(dispatch_error)
        .group(&GENERAL_GROUP)
        .help(&MY_HELP);
    framework.configure(
        Configuration::new()
            .with_whitespace(true)
            .prefix(&config.discord_bot_prefix),
    );

    let mut client = Client::builder(&config.discord_bot_token, GatewayIntents::default())
        .framework(framework)
        .event_handler(Handler::new())
        .await
        .expect("クライアントの作成中にエラーが発生しました");

    {
        let mut data = client.data.write().await;
        data.insert::<SendRules>(Arc::new(vec![
            Box::new(ChatRule),
            Box::new(RconRule),
            Box::new(LoginRule),
            Box::new(AdvancementRule),
            Box::new(ServerRule),
            Box::new(DeathRule),
        ]));
    }
    if let Err(reason) = client.start().await {
        eprintln!("クライアントの起動に失敗しました: {:?}", reason);
    }
}
