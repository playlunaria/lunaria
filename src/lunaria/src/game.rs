pub use commands::*;
pub use entity::*;
pub use events::*;
pub use handler::*;

mod commands;
mod entity;
mod events;
mod handler;

#[derive(Clone, Copy, Debug)]
pub enum GameStatus {
    Loading,
    Running,
    Stopped,
}

pub enum Command {
    StartGame(StartGame),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Event {
    GameStarted(GameStarted),
    GameFinished(GameFinished),
}
