use crate::{AppError, Context};

///sakana~~~
#[poise::command(slash_command, prefix_command)]
pub async fn fish(ctx: Context<'_>) -> Result<(), AppError> {
    ctx.send(|f| f.attachment("../assets/gifs/fish_takina.gif".into()))
        .await?;

    Ok(())
}
