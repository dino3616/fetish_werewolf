use poise::serenity_prelude as serenity;
use std::env;

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error(transparent)]
    Serenity(#[from] serenity::Error),
}

type Context<'a> = poise::Context<'a, (), AppError>;

#[poise::command(prefix_command, hide_in_help)]
async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<(), AppError> {
    poise::builtins::register_application_commands(ctx, global).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
async fn add(
    ctx: Context<'_>,
    #[description = "First number to add"] first: i32,
    #[description = "Second number to add"] second: i32,
) -> Result<(), AppError> {
    poise::say_reply(ctx, format!("{}", first + second)).await?;
    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, (), AppError>) {
    log::error!("Error: {:?}", error);
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let options = poise::FrameworkOptions {
        commands: vec![register(), add()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!fetish".to_string()),
            ..Default::default()
        },
        on_error: |err| Box::pin(on_error(err)),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .token(token)
        .options(options)
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(|_, _, _| Box::pin(async { Ok(()) }));

    framework.run().await.unwrap();
}
