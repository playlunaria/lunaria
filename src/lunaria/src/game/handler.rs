use tokio::sync::broadcast::Sender;

use crate::error::{Code, Error, ErrorKind, Result};
use crate::event::{Command as CommandTrait, Event as EventTrait};
use crate::game::{Command, Event as GameEvent, Event, Game, StartGame};

pub struct GameCommandHandler {
    event_bus: Sender<GameEvent>,
    game: Option<Game>,
}

impl GameCommandHandler {
    pub fn new(event_bus: Sender<GameEvent>) -> Self {
        Self {
            event_bus,
            game: None,
        }
    }

    pub fn handle(&mut self, command: Command) -> Result<()> {
        match command {
            Command::StartGame(payload) => self.handle_start_game(payload),
        }
    }

    fn handle_start_game(&mut self, command: StartGame) -> Result<()> {
        if self.game.is_some() {
            return Err(Error::new(
                Code::new("LUN0002"),
                ErrorKind::InvalidCommand,
                "A game is already running",
            ));
        }

        let mut game = Game::default();
        let events = command.handle(&game)?;

        for event in events {
            event.apply(&mut game);
            if let Err(_error) = self.event_bus.send(Event::GameStarted(event)) {
                // TODO: Log error
            }
        }

        self.game = Some(game);

        Ok(())
    }
}
