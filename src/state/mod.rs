use std::sync::Mutex;

#[derive(Debug, PartialEq)]
pub enum GameState {
    None,
    Preparing,
    Discussion,
    Voting,
    Evening,
}

impl GameState {
    pub fn transition_to(
        game_state: &Mutex<GameState>,
        next_state: GameState,
    ) -> anyhow::Result<()> {
        let Ok(mut game_state) = game_state.lock() else {
            anyhow::bail!("Failed to lock game state");
        };
        *game_state = next_state;

        Ok(())
    }

    pub fn not_equal(game_state: &Mutex<GameState>, comparison: GameState) -> anyhow::Result<bool> {
        let Ok(game_state) = game_state.lock() else {
            anyhow::bail!("Failed to lock game state");
        };
        Ok(*game_state != comparison)
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::None
    }
}
