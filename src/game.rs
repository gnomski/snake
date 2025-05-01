use crate::apples_system;
use crate::apples_system::check_apple_collision;
use crate::movement_system;
use crate::snake;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(snake::SnakeSegments::default())
            .insert_resource(snake::LastPositions::default())
            .insert_resource(snake::PendingTail::default())
            .insert_resource(snake::SnakeSpeed(100.0))
            .add_systems(Startup, setup_camera)
            .add_systems(Startup, snake::spawn_snake_head)
            .add_systems(Startup, apples_system::spawn_apple)
            .add_systems(
                Update,
                (movement_system::handle_input, movement_system::move_snake),
            )
            .add_systems(Update, check_apple_collision)
            .add_systems(Update, snake::process_tail)
            .add_systems(Update, (snake::track_head_positions, snake::update_body_positions));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
