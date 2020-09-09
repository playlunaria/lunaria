use std::time::Duration;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugin(bevy::type_registry::TypeRegistryPlugin::default())
        .add_plugin(bevy::core::CorePlugin::default())
        .add_plugin(bevy::transform::TransformPlugin::default())
        .add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
        .add_plugin(bevy::app::ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(1.0 / 60.0),
        ))
        .run();
}
