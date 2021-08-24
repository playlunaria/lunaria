use bevy::prelude::*;

use crate::scene::main_menu::MainMenu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Game,
}

pub fn run_app() {
    App::build()
        // Built-in plugins
        .add_plugins(DefaultPlugins)
        // Initial state of the game
        .add_state(AppState::MainMenu)
        // Background color
        .insert_resource(ClearColor(Color::hsl(231.0, 0.15, 0.18)))
        // Scenes
        .add_plugin(MainMenu)
        .run();
}
