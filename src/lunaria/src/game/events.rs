use crate::event::Event;
use crate::game::{Game, GameState};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct GameStarted {}

impl Event<Game> for GameStarted {
    fn apply(&self, aggregate: &mut Game) {
        aggregate.set_state(GameState::Started);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct GameFinished {}

impl Event<Game> for GameFinished {
    fn apply(&self, aggregate: &mut Game) {
        aggregate.set_state(GameState::Finished);
    }
}
