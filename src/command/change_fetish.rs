use poise::serenity_prelude as serenity;
use std::{collections::HashMap, sync::Mutex};

use crate::{context::Context, model::Attender, state::GameState};

/// Changes your fetish.
#[poise::command(prefix_command, slash_command)]
pub async fn change_fetish(
    ctx: Context<'_>,
    #[description = "New fetish"] new_fetish: String,
) -> anyhow::Result<()> {
    let game_state = &ctx.data().game_state;
    if GameState::not_equal(game_state, GameState::Preparing)? {
        anyhow::bail!("The game has already started or are not been held. Please type `/hold_fetish` to start a new game.");
    }

    let attenders = &ctx.data().attenders;
    let (old_fetish, new_fetish) = replace_fetish(attenders, ctx.author().id, new_fetish.clone())?;

    ctx.send(|rep| {
        rep.embed(|emb| {
            emb.title("Your fetish has been changed!")
                .description(format!("```diff\n- {}\n+ {}\n```", old_fetish, new_fetish))
                .color(serenity::Color::from_rgb(0, 255, 190))
        })
        .ephemeral(true)
    })
    .await?;

    Ok(())
}

fn replace_fetish(
    attenders: &Mutex<HashMap<serenity::UserId, Attender>>,
    id: serenity::UserId,
    fetish: String,
) -> anyhow::Result<(String, String)> {
    let Ok(mut attenders) = attenders.lock() else {
        anyhow::bail!("Failed to lock attenders");
    };

    let old_fetish: String;
    let new_fetish: String;
    if let Some(attender) = attenders.get_mut(&id) {
        old_fetish = attender.fetish.clone();
        new_fetish = fetish.clone();
        attender.fetish = fetish;
    } else {
        anyhow::bail!("You have not joined the game. Please type `/join` to join the game.");
    }

    Ok((old_fetish, new_fetish))
}
