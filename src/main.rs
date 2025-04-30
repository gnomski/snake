mod game;
mod movement_system;

use bevy::{app::App, DefaultPlugins};

fn main() {
    App:: new()
    .add_plugins(DefaultPlugins)
    .add_plugins(game::GamePlugin)
    .run();
}
