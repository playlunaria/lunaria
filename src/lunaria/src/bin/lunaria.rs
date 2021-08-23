use bevy::prelude::*;
use tokio::sync::broadcast;

use lunaria::plugin::api::Api;

#[tokio::main]
async fn main() {
    let (command_tx, command_rx) = broadcast::channel(256);

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(Api::new(command_tx))
        .run();
}
