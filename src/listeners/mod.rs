//! functions which react to non-command events

use poise::serenity_prelude::{self, Message};

use crate::{AppError, Data};

pub async fn listen_massage(
    ctx: &serenity_prelude::Context,
    _framework: poise::FrameworkContext<'_, Data, AppError>,
    _user_data: &Data,
    message: &Message,
) -> Result<(), AppError> {
    // reply to user who mentioned this bot
    if message.mentions_me(ctx).await? {
        let reply_text = format!("Hello {}!", message.author.name);
        message.reply_mention(ctx, reply_text).await?;
    }

    Ok(())
}
