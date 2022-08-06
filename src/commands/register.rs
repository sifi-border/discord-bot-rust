use crate::{AppError, Context};

///Registers application commands in this guild or globally
///Run with no arguments to register in guild, run with argument "global" to register globally.
#[poise::command(prefix_command, hide_in_help)]
pub async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<(), AppError> {
    poise::builtins::register_application_commands(ctx, global).await?;
    Ok(())
}
