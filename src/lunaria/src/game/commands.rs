use crate::error::Result;
use crate::event::Command;
use crate::game::{Game, GameStarted, GameState};

pub struct StartGame;

impl Command<Game, GameStarted> for StartGame {
    fn handle(self, aggregate: &Game) -> Result<Vec<GameStarted>> {
        match aggregate.state() {
            GameState::Initialized => Ok(vec![GameStarted {}]),
            _ => Ok(Vec::new()),
        }
    }
}
