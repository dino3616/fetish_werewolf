use poise::{serenity_prelude as serenity, FrameworkError::*};
use std::fmt::Display;

use crate::context::{Context, Data};

pub async fn on_error(error: poise::FrameworkError<'_, Data, anyhow::Error>) {
    log::error!("Error: {:?}", error);

    let result = match error {
        Command { error, ctx, .. } => reply_error(ctx, error).await,
        _ => {
            if let Some(ctx) = error.ctx() {
                reply_error(ctx, error).await
            } else {
                Ok(())
            }
        }
    };

    if let Err(err) = result {
        log::error!("Error occurred when sending error message: {:?}", err);
    }
}

async fn reply_error<'a>(ctx: Context<'a>, error: impl Display) -> anyhow::Result<()> {
    let error = if error.to_string().ends_with('.') {
        error.to_string()
    } else {
        format!("{}.", error)
    };

    ctx.send(|rep| {
        rep.embed(|emb| {
            emb.title("Oops! something went wrong! :-(")
                .description(format!("```{}```", error))
                .color(serenity::Color::RED)
        })
        .ephemeral(true)
    })
    .await
    .map(|_| ())?;

    Ok(())
}
