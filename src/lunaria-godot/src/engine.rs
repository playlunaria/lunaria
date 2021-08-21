use gdnative::prelude::*;
use tokio::sync::{mpsc, watch};

use lunaria::command::Command;
use lunaria::engine::Engine;
use lunaria::event::Event;

type CommandQueue = mpsc::Receiver<Command>;
type EventQueue = watch::Sender<Event>;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct EngineSingleton {
    _engine: Engine,
    #[allow(dead_code)] // TODO: Remove when switching to synchronous start command
    command_queue: CommandQueue,
    event_queue: EventQueue,
}

#[methods]
impl EngineSingleton {
    fn new(_owner: &Node) -> Self {
        let (command_sender, command_receiver) = mpsc::channel(256);
        let (event_sender, event_receiver) = watch::channel(Event::None);

        let engine = match Engine::new(command_sender, event_receiver) {
            Ok(engine) => engine,
            Err(error) => panic!("{}", error),
        };

        Self {
            _engine: engine,
            command_queue: command_receiver,
            event_queue: event_sender,
        }
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Engine ready");
    }

    #[export]
    fn on_game_status_running(&self, _owner: TRef<Node>) {
        if self.event_queue.send(Event::GameStarted).is_err() {
            // TODO: Log error
        }
    }

    #[export]
    fn on_game_status_stopped(&self, _owner: TRef<Node>) {
        if self.event_queue.send(Event::GameFinished).is_err() {
            // TODO: Log error
        }
    }
}
