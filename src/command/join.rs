use poise::serenity_prelude as serenity;
use std::{collections::HashMap, sync::Mutex};

use crate::{context::Context, model::Attender, state::GameState};

/// Joins a fetish werewolf.
#[poise::command(prefix_command, slash_command)]
pub async fn join(
    ctx: Context<'_>,
    #[description = "Your fetish"] fetish: String,
) -> anyhow::Result<()> {
    let game_state = &ctx.data().game_state;
    if GameState::not_equal(game_state, GameState::Preparing)? {
        anyhow::bail!("The game has already started or are not been held. Please type `/hold_fetish` to start a new game.");
    }

    let attenders = &ctx.data().attenders;
    insert_to_attenders(
        attenders,
        ctx.author().id,
        ctx.author().name.clone(),
        fetish.clone(),
    )?;

    ctx.send(|rep| {
        rep.embed(|emb| {
            emb.title("You are joined Fetish Werewolf!")
                .description(format!("Your fetish has been registered as `{}`.", &fetish))
                .color(serenity::Color::from_rgb(0, 255, 190))
        })
        .ephemeral(true)
    })
    .await?;

    Ok(())
}

fn insert_to_attenders(
    attenders: &Mutex<HashMap<serenity::UserId, Attender>>,
    id: serenity::UserId,
    name: String,
    fetish: String,
) -> anyhow::Result<()> {
    let Ok(mut attenders) = attenders.lock() else {
        anyhow::bail!("Failed to lock attenders");
    };
    if attenders.contains_key(&id) {
        anyhow::bail!("You have already joined the game. If you want to change your fetish, please type `/change_fetish`.");
    }

    attenders.insert(id, Attender::new(id, name, fetish));

    Ok(())
}
