use gdnative::prelude::*;
use tokio::sync::watch::{channel, Sender};

use lunaria::engine::Engine;
use lunaria::game::GameStatus;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct EngineSingleton {
    _engine: Engine,
    game_status_sender: Sender<GameStatus>,
}

#[methods]
impl EngineSingleton {
    fn new(_owner: &Node) -> Self {
        let (sender, receiver) = channel(GameStatus::Stopped);

        let engine = match Engine::new(receiver) {
            Ok(engine) => engine,
            Err(error) => panic!("{}", error),
        };

        Self {
            _engine: engine,
            game_status_sender: sender,
        }
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Engine ready");
    }

    #[export]
    fn on_game_status_running(&self, _owner: TRef<Node>) {
        if self.game_status_sender.send(GameStatus::Running).is_err() {
            // TODO: Log error
        }
    }

    #[export]
    fn on_game_status_stopped(&self, _owner: TRef<Node>) {
        if self.game_status_sender.send(GameStatus::Stopped).is_err() {
            // TODO: Log error
        }
    }
}
