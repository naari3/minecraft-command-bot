use log::{error, info};
use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::domains::rcon_client::RCONClient;

#[command]
#[description = "コマンドを送信する"]
#[usage = "[cmd...]"]
#[allowed_roles("cmd")]
async fn cmd(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let cmd = args.rest();
    info!("Receive cmd: `{cmd}`");
    let mut client = match RCONClient::new().await {
        Ok(c) => c,
        Err(err) => {
            error!("Cannot get connect `{err}`");
            msg.reply_ping(&ctx.http, format!("{err}")).await?;
            return Err(err)?;
        }
    };
    let response = client.cmd(cmd).await?;
    info!("Receive response: `{response}`");

    msg.reply_ping(&ctx.http, "success").await?;

    let chars: Vec<char> = response.chars().collect();
    let response_queue = &mut chars
        .chunks(2000 - 6)
        .map(|chunk| chunk.iter().collect::<String>());
    for response in response_queue {
        msg.channel_id
            .say(&ctx.http, format!("```{response}```"))
            .await?;
    }
    info!("Success cmd: `{cmd}`");

    Ok(())
}
