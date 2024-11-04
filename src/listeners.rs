use std::{
    sync::{atomic::Ordering, Arc},
    time::Duration,
};

use linemux::MuxedLines;
use log::{debug, error, info};
use regex::Regex;
use serenity::{
    all::{ActivityData, FullEvent},
    client::Context,
    model::{channel::Reaction, id::ChannelId},
};

use crate::{
    config::Config,
    domains::{ping_client::PingClient, rcon_client::RCONClient, send_rule::SendRule},
    error::Error,
    minecraft_line::MinecraftLine,
    Data,
};

pub async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            info!("{} is connected!", data_about_bot.user.name);
            let ctx = ctx.clone();
            if !data.is_loop_running.load(Ordering::Relaxed) {
                let config = Config::get();

                let ctx1 = ctx.clone();
                let send_rules = data.send_rules.clone();
                tokio::spawn(async move {
                    info!("Start tailing log");
                    let path = if let Some(path) = &config.minecraft_log_path {
                        path.clone()
                    } else {
                        info!("minecraft_log_path is not set");
                        return;
                    };
                    let channel_id: u64 = if let Some(channel_id) = &config.minecraft_log_channel_id
                    {
                        channel_id.parse::<_>().unwrap()
                    } else {
                        info!("minecraft_log_channel_id is not set");
                        return;
                    };
                    match tail_log(&ctx1, path, channel_id, send_rules).await {
                        Ok(_) => {}
                        Err(err) => {
                            log::error!("{err}");
                        }
                    }
                });

                let ctx2 = ctx.clone();
                tokio::spawn(async move {
                    loop {
                        match set_status(&ctx2).await {
                            Ok(_) => {}
                            Err(err) => {
                                log::error!("{err}");
                            }
                        };
                        tokio::time::sleep(Duration::from_secs(60)).await;
                    }
                });

                data.is_loop_running.swap(true, Ordering::Relaxed);
            }
        }
        FullEvent::ReactionAdd { add_reaction } => {
            match whitelist_add(ctx.clone(), add_reaction.clone()).await {
                Ok(_) => {}
                Err(err) => {
                    log::error!("{err}");
                }
            }
        }
        FullEvent::CacheReady { guilds: _ } => {}
        _ => {}
    }
    Ok(())
}

async fn whitelist_add(ctx: Context, add_reaction: Reaction) -> Result<(), Error> {
    if !add_reaction.emoji.unicode_eq("❤\u{fe0f}") {
        return Ok(());
    }

    if let Some(user_id) = add_reaction.user_id {
        debug!("reaction added by user_id: {}", user_id);
        if let Some(guild_id) = add_reaction.guild_id {
            let roles = guild_id.roles(&ctx.http).await?;
            debug!("roles: {:?}", roles);
            if let Some(cmd_role) = roles.values().find(|role| role.name == "cmd") {
                let have_cmd = user_id
                    .to_user(&ctx.http)
                    .await?
                    .has_role(&ctx.http, guild_id, cmd_role)
                    .await?;
                debug!("have_cmd: {}", have_cmd);
                if have_cmd {
                    let mut client = RCONClient::new().await?;
                    debug!("client");
                    let message = add_reaction.message(&ctx.http).await?;
                    debug!("message");
                    let result = client
                        .cmd(&format!("whitelist add {}", message.content))
                        .await?;
                    debug!("result");
                    message.reply_ping(&ctx.http, format!("`{result}`")).await?;
                    debug!("whitelist add result: {}", result);
                }
            }
        }
    };
    Ok(())
}

async fn tail_log(
    ctx: &Context,
    path: String,
    channel_id: u64,
    send_rules: Arc<Vec<Box<dyn SendRule + Send + Sync>>>,
) -> Result<(), Error> {
    // support minecraft log format (original, forge)
    // eg. [12:34:56] [Server thread/INFO]: <player> message
    // eg. [12:34:56] [Server thread/INFO] [minecraft/Server]: <player> message
    // eg. [31Dec2023 03:57:20.886] [Server thread/INFO] [minecraft/Server]: <player> message
    let log_re = Regex::new(r"\[(?P<time>[^\]]+)] \[(?P<caused_at>[^/]+)/(?P<level>[^\]]+)](?: \[[^\]]+])?: (?P<message>.*)").unwrap();

    let mut lines = MuxedLines::new().unwrap();
    lines.add_file(path).await.unwrap();
    while let Ok(Some(line)) = lines.next_line().await {
        debug!("new line: {:?}", line);
        if let Some(line) = log_re.captures(line.line()).map(|caps| {
            let time = caps.name("time").map_or("", |m| m.as_str());
            let caused_at = caps.name("caused_at").map_or("", |m| m.as_str());
            let level = caps.name("level").map_or("", |m| m.as_str());
            let message = caps.name("message").map_or("", |m| m.as_str());

            MinecraftLine::new(time.into(), caused_at.into(), level.into(), message.into())
        }) {
            debug!("captured: {:?}", line);
            for rule in send_rules.iter() {
                if let Some(msg) = rule.send(&line) {
                    debug!("matched. trying to send message \"{:?}\"", msg);
                    match ChannelId::new(channel_id).say(&ctx, msg).await {
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

async fn set_status(ctx: &Context) -> Result<(), Error> {
    let ping = PingClient::new().await;
    let status = ping.ping().await?;
    ctx.set_activity(Some(ActivityData::playing(format!(
        "{} player(s) online",
        status.players.online
    ))));
    Ok(())
}
