use bevy::prelude::*;
use crate::movement_system;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_camera)
            .add_systems(Startup, spawn_snake_head)
            .add_systems(Update, movement_system::snake_movement);
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
