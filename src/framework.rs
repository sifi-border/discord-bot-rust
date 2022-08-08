use super::{commands::*, listeners};
use crate::{on_error, serenity, AppError};

// TODO! 用途が出来たらしかるべき場所へ移す
#[derive(Debug)]
pub struct Data {}

async fn event_listener(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    framework: poise::FrameworkContext<'_, Data, AppError>,
    user_data: &Data,
) -> Result<(), AppError> {
    match event {
        poise::Event::Message { new_message } => {
            listeners::listen_massage(ctx, framework, user_data, new_message).await?;
        }
        _ => {}
    }
    Ok(())
}

pub async fn run_framework(token: &str) -> Result<(), AppError> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![register(), ping(), fish(), dice()],
            listener: |ctx, event, framework, user_data| {
                Box::pin(event_listener(ctx, event, framework, user_data))
            },
            on_error: |ctx| Box::pin(on_error(ctx)),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("::".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .token(token)
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }))
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
