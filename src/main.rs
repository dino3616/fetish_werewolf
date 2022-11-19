mod command;
mod error;

use poise::serenity_prelude as serenity;
use std::env;

type Context<'a> = poise::Context<'a, (), anyhow::Error>;

#[poise::command(prefix_command, slash_command)]
async fn add(
    ctx: Context<'_>,
    #[description = "First number to add"] first: i32,
    #[description = "Second number to add"] second: i32,
) -> anyhow::Result<()> {
    poise::say_reply(ctx, format!("{}", first + second)).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let options = poise::FrameworkOptions {
        commands: vec![command::register(), add()],
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
        .user_data_setup(|_, _, _| Box::pin(async { Ok(()) }));

    framework.run().await?;
    Ok(())
}
