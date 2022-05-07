use serenity::{
    async_trait,
    client::{Context, EventHandler},
    framework::standard::CommandResult,
    model::{channel::Reaction, prelude::Ready},
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
                    let result = client.cmd(&format!("whitelist add {}", "naarisan")).await?;
                    let message = add_reaction.message(&ctx.http).await?;
                    message.reply_ping(&ctx.http, format!("`{result}`")).await?;
                }
            }
        }
    };
    Ok(())
}
