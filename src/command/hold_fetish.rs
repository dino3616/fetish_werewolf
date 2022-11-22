use poise::serenity_prelude as serenity;

use crate::{context::Context, state::GameState};

/// Holds a fetish werewolf.
#[poise::command(prefix_command, slash_command)]
pub async fn hold_fetish(ctx: Context<'_>) -> anyhow::Result<()> {
    let game_state = &ctx.data().game_state;
    if GameState::not_equal(game_state, GameState::None)? {
        anyhow::bail!("Game is already in progress. Please wait until the current game is finished or type `/close_fetish` command to close the game.");
    }

    GameState::transition_to(&game_state, GameState::Preparing)?;

    ctx.send(|rep| {
        rep.embed(|emb| {
            emb.title(format!(
                "Fetish Werewolf was held by {}!",
                ctx.author().name
            ))
            .description("If you want to join the game, please type `/join` command.")
            .thumbnail(ctx.author().face())
            .color(serenity::Color::from_rgb(0, 255, 190))
        })
    })
    .await?;

    Ok(())
}
