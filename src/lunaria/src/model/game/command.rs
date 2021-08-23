use crate::error::{Code, Error, ErrorKind, Result};
use crate::event::Command;
use crate::model::game::entity::{Game, GameState};
use crate::model::game::event::GameStarted;

pub struct StartGame {}

impl Command<Game, GameStarted> for StartGame {
    fn handle(self, aggregate: &Game) -> Result<Vec<GameStarted>> {
        if aggregate.state == GameState::Initialized {
            Ok(vec![GameStarted { id: aggregate.id }])
        } else {
            Err(Error::new(
                Code::new(""),
                ErrorKind::InvalidCommand,
                "The game is already running or has already finished",
            ))
        }
    }
}
