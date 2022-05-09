use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::standard::{macros::hook, CommandResult, DispatchError},
    model::{
        channel::{Message, Reaction},
        prelude::Ready,
    },
};

use crate::domains::rcon_client::RCONClient;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        match whitelist_add(ctx, add_reaction).await {
            Ok(_) => {}
            Err(err) => {
                log::error!("{err}");
            }
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

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
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
