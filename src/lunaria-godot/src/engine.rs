use gdnative::prelude::*;
use tokio::sync::{broadcast, watch};

use lunaria::command::Command;
use lunaria::engine::Engine;
use lunaria::event::Event;

type CommandQueue = broadcast::Receiver<Command>;
type EventQueue = watch::Sender<Event>;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct EngineSingleton {
    _engine: Engine,
    command_queue: CommandQueue,
    event_queue: EventQueue,
}

#[methods]
impl EngineSingleton {
    fn new(_owner: &Node) -> Self {
        let (command_sender, command_receiver) = broadcast::channel(256);
        let (event_sender, _event_receiver) = watch::channel(Event::None);

        let engine = match Engine::new(command_sender) {
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
    fn _process(&mut self, owner: TRef<Node>, _delta: f32) {
        while let Ok(command) = self.command_queue.try_recv() {
            match command {
                Command::StartGame => self.start_game(owner),
            }
        }
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

    fn start_game(&self, owner: TRef<Node>) {
        if let Some(tree_ref) = owner.get_tree() {
            let tree = unsafe { tree_ref.assume_safe() };

            if let Err(error) = tree.change_scene("res://scenes/game/Game.tscn") {
                panic!("Failed to switch to game scene. {}", error);
            }
        }
    }
}
