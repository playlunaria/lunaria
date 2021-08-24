use bevy::prelude::*;

use crate::app::AppState;

const BACKGROUND_COLOR: Color = Color::hsl(231.0, 0.15, 0.18);
const TEXT_COLOR: Color = Color::WHITE;

pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::MainMenu).with_system(render_main_menu.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu).with_system(close_main_menu.system()),
        );
    }

    fn name(&self) -> &str {
        "scene/title_screen"
    }
}

fn render_main_menu(
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
            parent.spawn_bundle(padding(
                materials.add(BACKGROUND_COLOR.into()),
                Val::Auto,
                Val::Px(128.0),
            ));
            parent.spawn_bundle(ImageBundle {
                material: materials.add(asset_server.load("brand/Logo.png").into()),
                ..Default::default()
            });
            parent.spawn_bundle(heading(
                asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                "Lunaria",
            ));
            parent.spawn_bundle(padding(
                materials.add(BACKGROUND_COLOR.into()),
                Val::Auto,
                Val::Px(128.0),
            ));
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(256.0), Val::Px(64.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        ..Default::default()
                    },
                    material: materials.add(BACKGROUND_COLOR.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(label(
                        asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                        &format!("Version {}", env!("CARGO_PKG_VERSION")),
                    ));
                    parent.spawn_bundle(label(
                        asset_server.load("fonts/JetBrainsMono-Regular.ttf"),
                        "API 0.2.1",
                    ));
                });
        });
}

fn close_main_menu() {}

fn padding(material: Handle<ColorMaterial>, width: Val, height: Val) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(width, height),
            ..Default::default()
        },
        material,
        ..Default::default()
    }
}

fn heading(font: Handle<Font>, text: &str) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            text,
            TextStyle {
                font,
                font_size: 64.0,
                color: TEXT_COLOR,
            },
            Default::default(),
        ),
        ..Default::default()
    }
}

fn label(font: Handle<Font>, text: &str) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            text,
            TextStyle {
                font,
                font_size: 16.0,
                color: TEXT_COLOR,
            },
            Default::default(),
        ),
        ..Default::default()
    }
}
