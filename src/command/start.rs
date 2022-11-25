use poise::serenity_prelude as serenity;
use std::{collections::HashMap, sync::Mutex};

use crate::{
    context::Context,
    model::{Attender, Job},
    state::GameState,
};

/// Changes your fetish.
#[poise::command(prefix_command, slash_command)]
pub async fn start(ctx: Context<'_>) -> anyhow::Result<()> {
    let game_state = &ctx.data().game_state;
    if GameState::not_equal(game_state, GameState::Preparing)? {
        anyhow::bail!("The game has already started or are not been held. Please type `/hold_fetish` to start a new game.");
    }

    GameState::transition_to(&game_state, GameState::Discussion)?;

    ctx.send(|rep| {
        rep.embed(|emb| {
            emb.title(format!(
                "Fetish Werewolf was started by {}!",
                ctx.author().name
            ))
            .thumbnail(ctx.author().face())
            .color(serenity::Color::from_rgb(0, 255, 190))
        })
    })
    .await?;

    let attenders = &ctx.data().attenders;
    let werewolves = decide_werewolves(attenders)?;

    ctx.send(|rep| {
        rep.embed(|emb| {
            emb.title("werewolves have been decided!")
                .description(format!(
                    "The fetishes of werewolves are here!\n```{}```",
                    werewolves
                        .iter()
                        .map(|attender| {
                            let mut fetish = attender.1.clone();
                            fetish.insert_str(0, "- ");

                            fetish
                        })
                        .collect::<Vec<String>>()
                        .join("\n")
                ))
                .color(serenity::Color::from_rgb(0, 255, 190))
        })
    })
    .await?;

    GameState::transition_to(game_state, GameState::Discussion)?;

    ctx.send(|rep| {
        rep.embed(|emb| {
            emb.title("Fetish Werewolf has started!\nLet the discussion begin!")
                .color(serenity::Color::from_rgb(0, 255, 190))
        })
    })
    .await?;

    Ok(())
}

fn decide_werewolves(
    attenders: &Mutex<HashMap<serenity::UserId, Attender>>,
) -> anyhow::Result<HashMap<serenity::UserId, String>> {
    let Ok(mut attenders) = attenders.lock() else {
        anyhow::bail!("Failed to lock attenders");
    };

    let werewolf_num = if attenders.len() <= 6 { 1 } else { 2 };

    let mut werewolves = HashMap::new();
    for (_, (_, attender)) in (0..werewolf_num).zip(attenders.iter_mut()) {
        attender.job = Job::Werewolf;
        werewolves.insert(attender.id, attender.fetish.clone());
    }

    Ok(werewolves)
}
