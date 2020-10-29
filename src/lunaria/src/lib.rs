use bevy::prelude::*;

pub mod plugins;

pub fn run_app() {
    App::build()
        .add_default_plugins()
        .add_plugin(plugins::api::Api)
        .run();
}
