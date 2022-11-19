#[derive(Debug)]
pub enum GameState {
    None,
    Preparing,
    Discussion,
    Voting,
    Evening,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::None
    }
}
