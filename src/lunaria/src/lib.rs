use bevy::prelude::*;

pub mod plugins;

pub fn run_app() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(plugins::api::Api)
        .run();
}
