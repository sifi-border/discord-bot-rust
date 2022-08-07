use super::commands::*;
use crate::{on_error, serenity, AppError};

pub async fn run_framework(token: &str) -> Result<(), AppError> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![register(), ping(), fish(), dice()],
            on_error: |ctx| Box::pin(on_error(ctx)),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("::".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .token(token)
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok({}) }))
        .intents(
            serenity::GatewayIntents::non_privileged()
                | serenity::GatewayIntents::GUILD_MEMBERS
                | serenity::GatewayIntents::GUILD_PRESENCES
                | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .run()
        .await?;

    Ok(())
}
