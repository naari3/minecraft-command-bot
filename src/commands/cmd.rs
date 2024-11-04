use log::{error, info};

use crate::{domains::rcon_client::RCONClient, error::Error, Context};

#[poise::command(slash_command, prefix_command)]
pub async fn cmd(
    ctx: Context<'_>,
    #[description = "Specific command to run on the server"] command: String,
) -> Result<(), Error> {
    info!("Receive cmd: {} `{command}`", ctx.author());
    let partial_guild = if let Some(partial_guild) = ctx.partial_guild().await {
        partial_guild
    } else {
        error!("Cannot get partial_guild");
        ctx.reply("Cannot get partial_guild").await?;
        return Ok(());
    };
    let cmd_role_id = if let Some(cmd_role) = partial_guild.role_by_name("cmd") {
        cmd_role.id
    } else {
        error!("Cannot get cmd role");
        ctx.reply("Cannot get cmd role").await?;
        return Ok(());
    };
    let has_cmd_role = ctx
        .author()
        .has_role(ctx.http(), partial_guild.id, cmd_role_id)
        .await?;
    if !has_cmd_role {
        error!("{} does not have cmd role", ctx.author());
        ctx.reply("You do not have cmd role").await?;
        return Ok(());
    }

    let mut client = match RCONClient::new().await {
        Ok(c) => c,
        Err(err) => {
            error!("Cannot get connect `{err}`");
            ctx.reply(format!("{err}")).await?;
            return Err(err)?;
        }
    };
    let response = client.cmd(&command).await?;
    info!("Receive response: `{response}`");

    ctx.reply("success").await?;

    let chars: Vec<char> = response.chars().collect();
    let response_queue = &mut chars
        .chunks(2000 - 6)
        .map(|chunk| chunk.iter().collect::<String>());
    for response in response_queue {
        ctx.say(format!("```{response}```")).await?;
    }
    info!("Success cmd: {} `{command}`", ctx.author());

    Ok(())
}
