use bevy::prelude::*;
use tokio::sync::broadcast;

use crate::plugin::api::Api;
use crate::scene::main_menu::MainMenu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Game,
}

pub fn run_app() {
    let (command_tx, _command_rx) = broadcast::channel(256);

    App::build()
        // Built-in plugins
        .add_plugins(DefaultPlugins)
        // Initial state of the game
        .add_state(AppState::MainMenu)
        // Background color
        .insert_resource(ClearColor(Color::hsl(231.0, 0.15, 0.18)))
        // Scenes
        .add_plugin(MainMenu)
        // Custom plugins
        .add_plugin(Api::new(command_tx))
        .run();
}
