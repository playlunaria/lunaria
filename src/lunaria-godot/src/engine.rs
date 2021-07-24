use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct EngineSingleton {}

#[methods]
impl EngineSingleton {
    fn new(_owner: &Node) -> Self {
        Self {}
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Engine ready");
    }
}
