use gdnative::prelude::*;

use crate::engine::EngineSingleton;
use crate::scenes::game::GameScene;

mod engine;
mod scenes;

fn init(handle: InitHandle) {
    // Singletons
    handle.add_class::<EngineSingleton>();

    // Scenes
    handle.add_class::<GameScene>();
}

godot_init!(init);
