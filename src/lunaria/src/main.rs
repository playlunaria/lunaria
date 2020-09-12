use bevy::prelude::*;

use crate::grpc::GrpcPlugin;

mod grpc;

#[tokio::main]
async fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(GrpcPlugin)
        .add_resource(Counter { count: 0 })
        .add_startup_system(setup.system())
        .add_system(increase_counter_system.system())
        .add_system(print_counter_system.system())
        .run();
}

struct Counter {
    count: usize,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server
        .load("src/lunaria/assets/JetBrainsMono-Regular.ttf")
        .unwrap();

    commands
        .spawn(UiCameraComponents::default())
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "Counter: 0".to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
        });
}

fn increase_counter_system(mut counter: ResMut<Counter>) {
    counter.count += 1;
}

fn print_counter_system(counter: Res<Counter>, mut query: Query<&mut Text>) {
    for mut text in &mut query.iter() {
        text.value = format!("Counter: {}", counter.count);
    }
}
