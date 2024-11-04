use log::{error, info};
use poise::CreateReply;

use crate::domains::rcon_client::RCONClient;
use crate::error::Error;
use crate::Context;

#[poise::command(slash_command, prefix_command)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "Specific message to send to server"] message: String,
) -> Result<(), Error> {
    info!("Receive say: {} `{message}`", ctx.author());
    let mut client = match RCONClient::new().await {
        Ok(c) => c,
        Err(err) => {
            error!("Cannot get connect `{err}`");
            ctx.reply(format!("{err}")).await?;
            return Err(err)?;
        }
    };
    let name = ctx
        .author()
        .nick_in(ctx.http(), ctx.guild_id().unwrap_or_default())
        .await
        .unwrap_or_else(|| {
            ctx.author()
                .global_name
                .clone()
                .unwrap_or(ctx.author().name.clone())
        });
    let cmd = format!("say <{name}> {message}");
    let _ = client.cmd(&cmd).await?;

    let reply = ctx
        .send(CreateReply {
            content: Some("📨".to_string()),
            ephemeral: Some(true),
            ..Default::default()
        })
        .await?;
    reply.delete(ctx).await?;
    info!("Receive say: {} `{message}`", ctx.author());

    Ok(())
}
