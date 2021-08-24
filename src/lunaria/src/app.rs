use bevy::prelude::*;
use tokio::sync::broadcast;

use crate::command::CommandQueue;
use crate::plugin::api::Api;
use crate::scene::game::Game;
use crate::scene::main_menu::MainMenu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Game,
}

pub fn run_app() {
    let mut app = App::build();

    app
        // Built-in plugins
        .add_plugins(DefaultPlugins)
        // Initial state of the game
        .add_state(AppState::MainMenu)
        // Background color
        .insert_resource(ClearColor(Color::hsl(231.0, 0.15, 0.18)))
        // Scenes
        .add_plugin(Game)
        .add_plugin(MainMenu);

    let (command_tx, command_rx) = broadcast::channel(256);

    app.add_plugin(Api::new(command_tx.clone()));

    let command_queue = CommandQueue {
        sender: command_tx,
        receiver: command_rx,
    };

    app.insert_resource(command_queue);

    app.run();
}
