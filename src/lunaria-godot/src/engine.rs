use gdnative::prelude::*;

use lunaria::engine::Engine;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct EngineSingleton {
    _engine: Engine,
}

#[methods]
impl EngineSingleton {
    fn new(_owner: &Node) -> Self {
        let engine = match Engine::new() {
            Ok(engine) => engine,
            Err(error) => panic!("{}", error),
        };

        Self { _engine: engine }
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Engine ready");
    }
}
