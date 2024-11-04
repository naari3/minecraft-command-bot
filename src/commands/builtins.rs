use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn help(ctx: Context<'_>, command: Option<String>) -> Result<(), Error> {
    let configuration = poise::builtins::HelpConfiguration {
        // [configure aspects about the help message here]
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), configuration).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn pretty_help(ctx: Context<'_>, command: Option<String>) -> Result<(), Error> {
    let configuration = poise::builtins::PrettyHelpConfiguration {
        // [configure aspects about the help message here]
        ..Default::default()
    };
    poise::builtins::pretty_help(ctx, command.as_deref(), configuration).await?;
    Ok(())
}

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
