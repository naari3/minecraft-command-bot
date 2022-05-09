use config::Config;
use dotenv::dotenv;
use log::info;
use serenity::{
    framework::{standard::macros::group, StandardFramework},
    Client,
};

mod commands;
mod config;
mod domains;
mod error;
mod listeners;

use commands::{cmd::*, help::*, mcuuid::*};

use crate::listeners::{after_commands, dispatch_error};

#[group]
#[description("コマンドコマンド")]
#[summary("main")]
#[only_in(guilds)]
#[commands(cmd, mcuuid)]
struct General;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("minecraft_command_bot=info"),
    )
    .init();

    let config = Config::get();

    info!("Start command bot");

    let framework = StandardFramework::new()
        .configure(|config| config.prefix("\\"))
        // .before(before_commands)
        .after(after_commands)
        .on_dispatch_error(dispatch_error)
        .group(&GENERAL_GROUP)
        .help(&MY_HELP);

    let mut client = Client::builder(&config.discord_bot_token)
        .framework(framework)
        .event_handler(listeners::Handler)
        .await
        .expect("クライアントの作成中にエラーが発生しました");

    if let Err(reason) = client.start().await {
        eprintln!("クライアントの起動に失敗しました: {:?}", reason);
    }
}
