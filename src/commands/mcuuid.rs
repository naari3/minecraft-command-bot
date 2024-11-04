use log::info;
use uuid::Uuid;

use crate::domains::mojang_client::MojangClient;
use crate::error::Error;
use crate::Context;

#[poise::command(slash_command, prefix_command)]
pub async fn mcuuid(
    ctx: Context<'_>,
    #[description = "Specific name or uuid"] name_or_uuid: String,
) -> Result<(), Error> {
    info!("Receive mcuuid: {} `{name_or_uuid}`", ctx.author());
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

    ctx.reply(message).await?;

    info!("Success mcuuid: {} `{name_or_uuid}`", ctx.author());

    Ok(())
}
