use crate::{AppError, Context};

///ping pong
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), AppError> {
    ctx.say("pong!").await?;
    Ok(())
}
