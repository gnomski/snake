use crate::apples_system;
use crate::apples_system::check_apple_collision;
use crate::movement_system;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Startup, spawn_snake_head)
            .add_systems(Update, movement_system::snake_movement)
            .add_systems(Startup, apples_system::spawn_apple)
            .add_systems(Update, check_apple_collision);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

#[derive(Component)]
pub struct SnakeHead;

fn spawn_snake_head(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/snake_head.png"),
            custom_size: Some(Vec2::new(32., 32.)),
            ..default()
        },
        Transform::default(),
        GlobalTransform::default(),
        SnakeHead,
    ));
}
