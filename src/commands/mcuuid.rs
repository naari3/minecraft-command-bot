use log::info;
use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use uuid::Uuid;

use crate::domains::mojang_client::MojangClient;

#[command]
#[description = "usernameとuuidを相互変換する"]
#[usage = "[username | uuid]"]
async fn mcuuid(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name_or_uuid = args.single::<String>()?;
    info!("Receive mcuuid: {} `{name_or_uuid}`", msg.author.tag());
    let mut client = MojangClient::new();
    let result = match Uuid::parse_str(&name_or_uuid) {
        Ok(uuid) => client.uuid_to_name(&uuid).await?,
        Err(_) => client
            .username_to_uuid(&name_or_uuid)
            .await?
            .map(|u| u.as_hyphenated().to_string()),
    };

    info!("Receive result: `{:?}`", result);

    let message = match result {
        Some(result) => format!("=> `{result}`"),
        None => "not found".to_string(),
    };

    msg.reply_ping(&ctx.http, message).await?;

    info!("Success mcuuid: {} `{name_or_uuid}`", msg.author.tag());

    Ok(())
}
