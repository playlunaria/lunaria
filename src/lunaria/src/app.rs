use bevy::prelude::*;
use tokio::sync::broadcast;

use crate::plugin::api::Api;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
}

pub struct Lunaria;

impl Lunaria {
    pub fn run() {
        let (command_tx, command_rx) = broadcast::channel(256);

        App::build()
            .add_plugins(DefaultPlugins)
            .add_plugin(Api::new(command_tx))
            .add_state(AppState::MainMenu)
            .run();
    }
}
