mod command;
mod context;
mod error;
mod model;
mod state;

use poise::serenity_prelude as serenity;
use std::env;

use crate::context::Data;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let options = poise::FrameworkOptions {
        commands: vec![command::register(), command::help()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!fetish".to_string()),
            ..Default::default()
        },
        on_error: |err| Box::pin(error::on_error(err)),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .token(token)
        .options(options)
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(|_, _, _| Box::pin(async move { Ok(Data::new()) }));

    framework.run().await?;
    Ok(())
}
