use gdnative::prelude::*;

use crate::engine::EngineSingleton;

mod engine;

fn init(handle: InitHandle) {
    // Singletons
    handle.add_class::<EngineSingleton>();
}

godot_init!(init);
