use bevy::app::App;
use bevy::DefaultPlugins;

use crate::api::Api;

pub mod api;

pub fn run_app() {
    let api = initialize_api();

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(api)
        .run();
}

fn initialize_api() -> Api {
    Api::new()
}
