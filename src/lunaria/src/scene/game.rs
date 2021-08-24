use bevy::prelude::*;

use crate::app::AppState;
use crate::scene::BACKGROUND_COLOR;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(enter_game.system()))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(close_game.system()));
    }

    fn name(&self) -> &str {
        "scene/game"
    }
}

fn enter_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());

    // Root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            material: materials.add(BACKGROUND_COLOR.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Game",
                    TextStyle {
                        font: asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                        font_size: 64.0,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}

fn close_game(mut commands: Commands, mut entities: Query<Entity>) {
    for entity in entities.iter_mut() {
        commands.entity(entity).despawn();
    }
}
