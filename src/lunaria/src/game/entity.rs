use getset::{Getters, Setters};
use uuid::Uuid;

use crate::event::Aggregate;

#[derive(Getters, Setters)]
pub struct Game {
    #[getset(get = "pub", set = "pub(super)")]
    id: Uuid,

    #[getset(get = "pub", set = "pub(super)")]
    state: GameState,
}

pub enum GameState {
    Initialized,
    Started,
    Finished,
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
