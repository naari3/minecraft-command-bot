use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use linemux::MuxedLines;
use log::{debug, error, info};
use regex::Regex;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::standard::{macros::hook, CommandResult, DispatchError},
    model::{
        channel::{Message, Reaction},
        id::{ChannelId, GuildId},
        prelude::{Activity, Ready},
    },
};

use crate::{
    config::Config,
    domains::{ping_client::PingClient, rcon_client::RCONClient},
    globals::SendRules,
    minecraft_line::MinecraftLine,
};

pub struct Handler {
    is_loop_running: AtomicBool,
}

impl Handler {
    pub fn new() -> Self {
        Self {
            is_loop_running: AtomicBool::new(false),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        match whitelist_add(ctx, add_reaction).await {
            Ok(_) => {}
            Err(err) => {
                log::error!("{err}");
            }
        }
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        let ctx = Arc::new(ctx);
        if !self.is_loop_running.load(Ordering::Relaxed) {
            let config = Config::get();

            let ctx1 = Arc::clone(&ctx);
            tokio::spawn(async move {
                let path = if let Some(path) = &config.minecraft_log_path {
                    path.clone()
                } else {
                    return;
                };
                let channel_id: u64 = if let Some(channel_id) = &config.minecraft_log_channel_id {
                    channel_id.parse::<_>().unwrap()
                } else {
                    return;
                };
                match tail_log(Arc::clone(&ctx1), path, channel_id).await {
                    Ok(_) => {}
                    Err(err) => {
                        log::error!("{err}");
                    }
                }
            });

            let ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    match set_status(Arc::clone(&ctx2)).await {
                        Ok(_) => {}
                        Err(err) => {
                            log::error!("{err}");
                        }
                    };
                    tokio::time::sleep(Duration::from_secs(60)).await;
                }
            });

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}

async fn whitelist_add(ctx: Context, add_reaction: Reaction) -> CommandResult {
    if !add_reaction.emoji.unicode_eq("❤\u{fe0f}") {
        return Ok(());
    }
    if let Some(user_id) = add_reaction.user_id {
        if let Some(guild_id) = add_reaction.guild_id {
            let roles = ctx
                .cache
                .guild_field(guild_id, |guild| guild.roles.clone())
                .await
                .unwrap();
            if let Some(cmd_role) = roles.values().find(|role| role.name == "cmd") {
                let have_cmd = user_id
                    .to_user(&ctx.http)
                    .await?
                    .has_role(&ctx.http, guild_id, cmd_role)
                    .await?;
                if have_cmd {
                    let mut client = RCONClient::new().await?;
                    let message = add_reaction.message(&ctx.http).await?;
                    let result = client
                        .cmd(&format!("whitelist add {}", message.content))
                        .await?;
                    message.reply_ping(&ctx.http, format!("`{result}`")).await?;
                }
            }
        }
    };
    Ok(())
}

async fn tail_log(ctx: Arc<Context>, path: String, channel_id: u64) -> CommandResult {
    let log_re = Regex::new(r"^\[(.*)]\s\[([^/]*)/(.*)][^:]*:\s(.*)$").unwrap();
    let mut lines = MuxedLines::new().unwrap();
    lines.add_file(path).await.unwrap();
    while let Ok(Some(line)) = lines.next_line().await {
        debug!("{:?}", line);
        if let Some(line) = log_re.captures(&line.line()).map(|cap| {
            let time = cap
                .get(1)
                .map(|time| time.as_str().to_string())
                .unwrap_or("".to_string());
            let caused_at = cap
                .get(2)
                .map(|ca| ca.as_str().to_string())
                .unwrap_or("".to_string());
            let level = cap
                .get(3)
                .map(|l| l.as_str().to_string())
                .unwrap_or("".to_string());
            let message = cap
                .get(4)
                .map(|m| m.as_str().to_string())
                .unwrap_or("".to_string());

            MinecraftLine::new(time, caused_at, level, message)
        }) {
            let data_read = ctx.data.read().await;
            let send_rules_lock = data_read.get::<SendRules>().unwrap().clone();
            for rule in send_rules_lock.iter() {
                if let Some(msg) = rule.send(&line) {
                    match ChannelId(channel_id).say(&ctx, msg).await {
                        Ok(_) => {}
                        Err(err) => {
                            error!("{err}");
                        }
                    }
                }
            }
        };
    }
    Ok(())
}

async fn set_status(ctx: Arc<Context>) -> CommandResult {
    let ping = PingClient::new().await;
    let status = ping.ping().await?;
    ctx.set_activity(Activity::playing(format!(
        "{} player(s) online",
        status.players.online
    )))
    .await;
    Ok(())
}

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    log::error!("{:?}", error);
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
