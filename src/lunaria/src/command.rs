use bevy::prelude::*;
use tokio::sync::broadcast::{Receiver, Sender};

#[derive(Clone)]
pub enum Command {
    StartGame,
}

pub struct CommandQueue {
    pub sender: Sender<Command>,
    pub receiver: Receiver<Command>,
}

pub struct CommandReceiver {
    pub queue: Receiver<Command>,
}

impl FromWorld for CommandReceiver {
    fn from_world(world: &mut World) -> Self {
        let command_queue = world.get_resource::<CommandQueue>().unwrap();

        Self {
            queue: command_queue.sender.subscribe(),
        }
    }
}
