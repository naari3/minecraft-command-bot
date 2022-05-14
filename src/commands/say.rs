use log::{error, info};
use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::domains::rcon_client::RCONClient;

#[command]
#[description = "コマンドを送信する"]
#[usage = "[say ...]"]
async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let arg_msg = args.rest();
    info!("Receive say: {} `{arg_msg}`", msg.author.tag());
    let mut client = match RCONClient::new().await {
        Ok(c) => c,
        Err(err) => {
            error!("Cannot get connect `{err}`");
            msg.reply_ping(&ctx.http, format!("{err}")).await?;
            return Err(err)?;
        }
    };
    let name = msg
        .author
        .nick_in(&ctx.http, msg.guild_id.unwrap())
        .await
        .unwrap_or_else(|| msg.author.name.clone());
    let cmd = format!("say <{name}> {arg_msg}");
    let _ = client.cmd(&cmd).await?;

    info!("Receive say: {} `{arg_msg}`", msg.author.tag());

    Ok(())
}
