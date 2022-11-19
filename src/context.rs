use std::{collections::HashMap, sync::Mutex};

use crate::{model, state::GameState};

pub type Context<'a> = poise::Context<'a, Data, anyhow::Error>;

#[derive(Debug)]
pub struct Data {
    attenders: Mutex<HashMap<String, model::Attender>>,
    game_state: Mutex<GameState>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            attenders: Mutex::new(HashMap::new()),
            game_state: Mutex::new(GameState::default()),
        }
    }
}
