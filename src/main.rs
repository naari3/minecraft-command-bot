use config::Config;
use dotenv::dotenv;
use log::info;
use serenity::{
    client::Context,
    framework::{
        standard::{
            macros::{group, hook},
            CommandResult, DispatchError,
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

use commands::{cmd::*, help::*, mcuuid::*};

#[group]
#[description("コマンドコマンド")]
#[summary("main")]
#[only_in(guilds)]
#[commands(cmd, mcuuid)]
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

#[hook]
pub async fn after_commands(
    ctx: &Context,
    message: &Message,
    command_name: &str,
    command_result: CommandResult,
) {
    if let Err(err) = command_result {
        let _ = message.reply(&ctx.http, &err).await;
        if format!("{}", err).contains("不明なエラー") {
            println!(
                "[{}] {}の処理中にエラーが発生しました。\nerror: {}\nmessage: {}\nauthor: {} (id: {})\nguild_id: {:?}",
                message.timestamp,
                command_name,
                err,
                message.content,
                message.author.name,
                message.author.id.as_u64(),
                message.guild_id
            );
        }
    }
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
