use gdnative::prelude::*;
use strum::IntoEnumIterator;
use strum::{AsRefStr, EnumIter};

#[derive(AsRefStr, EnumIter, Debug)]
pub enum GameStatusSignal {
    Running,
    Stopped,
}

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct GameScene {}

#[methods]
impl GameScene {
    pub fn new(_owner: &Node) -> Self {
        Self {}
    }

    #[export]
    fn _ready(&mut self, owner: TRef<Node>) {
        self.connect_signals_to_engine(owner);
        owner.emit_signal(GameStatusSignal::Running.as_ref(), &[]);
    }

    #[export]
    fn _exit_tree(&mut self, owner: &Node) {
        owner.emit_signal(GameStatusSignal::Stopped.as_ref(), &[]);
    }

    fn register_signals(builder: &ClassBuilder<Self>) {
        for status in GameStatusSignal::iter() {
            builder.add_signal(Signal {
                name: status.as_ref(),
                args: &[],
            });
        }
    }

    fn connect_signals_to_engine(&self, owner: TRef<Node>) {
        let signals = vec![
            (GameStatusSignal::Running.as_ref(), "on_game_status_running"),
            (GameStatusSignal::Stopped.as_ref(), "on_game_status_stopped"),
        ];

        if let Some(engine) = &mut owner.get_node("/root/EngineSingleton") {
            let engine = unsafe { engine.assume_safe() };

            for (signal, method) in signals {
                match owner.connect(signal, engine, method, VariantArray::new_shared(), 0) {
                    Ok(_) => (),
                    Err(error) => {
                        godot_print!("Failed to connect game running signal to engine. {}", error);
                    }
                }
            }
        }
    }
}
