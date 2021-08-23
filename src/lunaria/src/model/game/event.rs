use uuid::Uuid;

use crate::event::Event;
use crate::model::game::entity::GameState;

use super::entity::Game;

pub enum GameEvent {
    GameStarted(GameStarted),
    GameFinished(GameFinished),
}

pub struct GameStarted {
    pub(super) id: Uuid,
}

impl Event<Game> for GameStarted {
    fn apply(&self, aggregate: &mut Game) {
        aggregate.state = GameState::Running;
    }
}

pub struct GameFinished {}

impl Event<Game> for GameFinished {
    fn apply(&self, aggregate: &mut Game) {
        aggregate.state = GameState::Finished;
    }
}
