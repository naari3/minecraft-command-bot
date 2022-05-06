use config::Config;
use dotenv::dotenv;
use log::info;
use serenity::{
    client::Context,
    framework::{
        standard::{
            macros::{group, hook},
            DispatchError,
        },
        StandardFramework,
    },
    model::channel::Message,
    Client,
};

mod commands;
mod config;
mod domains;
mod error;
mod listeners;

use commands::{cmd::*, help::*};

#[group]
#[description("コマンドコマンド")]
#[summary("main")]
#[only_in(guilds)]
#[commands(cmd)]
struct General;

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    println!("{:?}", error);
    match error {
        // DispatchError::CheckFailed(_, _) => todo!(),
        DispatchError::Ratelimited(info) => {
            // We notify them only once.
            if info.is_first_try {
                let _ = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        &format!("Try this again in {} seconds.", info.as_secs()),
                    )
                    .await;
            }
        }
        // DispatchError::CommandDisabled(_) => todo!(),
        // DispatchError::BlockedUser => todo!(),
        // DispatchError::BlockedGuild => todo!(),
        // DispatchError::BlockedChannel => todo!(),
        // DispatchError::OnlyForDM => todo!(),
        // DispatchError::OnlyForGuilds => todo!(),
        // DispatchError::OnlyForOwners => todo!(),
        DispatchError::LackingRole => {
            let _ = msg
                .channel_id
                .say(&ctx.http, "You have not enough role")
                .await;
        }
        // DispatchError::LackingPermissions(_) => todo!(),
        // DispatchError::NotEnoughArguments { min, given } => todo!(),
        // DispatchError::TooManyArguments { max, given } => todo!(),
        _ => {}
    };
}

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
        // .after(after_commands)
        // .before(before_commands)
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
