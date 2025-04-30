mod game;
mod movement_system;
mod apples_system;

use bevy::{app::App, DefaultPlugins};
use bevy::prelude::*;

fn main() {
    App:: new()
    .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
        filter: "info".into(),
        level: bevy::log::Level::INFO,
        custom_layer: |_| None,
    }))
    .add_plugins(game::GamePlugin)
    .run();
}
