mod commands;
mod framework;
use std::env;

#[macro_use]
extern crate log;

pub use poise::serenity_prelude as serenity;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    Serenity(#[from] serenity::Error),
}

type Context<'a> = poise::Context<'a, (), AppError>;

async fn on_error(error: poise::FrameworkError<'_, (), AppError>) {
    error!("{:?}", error);
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv::dotenv().ok();
    env_logger::init();
    env::set_current_dir(std::path::Path::new("src")).ok();
    let token = env::var("BOT_TOKEN").expect("BOT_TOKEN is not set");
    framework::run_framework(&token).await?;

    Ok(())
}
