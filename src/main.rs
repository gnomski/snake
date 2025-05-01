mod game;
mod movement_system;
mod apples_system;
mod snake;
mod constants;

use bevy::{app::App, DefaultPlugins};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "🐍 Solid Snake Game".to_string(),
                ..default()
            }),
            ..default()
        }).set(bevy::log::LogPlugin {
            filter: "info".into(),
            level: bevy::log::Level::INFO,
            custom_layer: |_| None,
        }))
        .add_plugins(game::GamePlugin)
        .run();
}

