use uuid::Uuid;

use crate::event::Aggregate;

#[derive(PartialEq)]
pub enum GameState {
    Initialized,
    Running,
    Finished,
}

pub struct Game {
    pub(super) id: Uuid,
    pub(super) state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Aggregate for Game {}

impl Default for Game {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            state: GameState::Initialized,
        }
    }
}
