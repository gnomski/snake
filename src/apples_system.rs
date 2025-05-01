use crate::snake::SnakeHead;
use crate::snake::PendingTail;

use bevy::prelude::*;
use rand::Rng;
use crate::constants::*;

#[derive(Component)]
pub struct Apple;

pub fn spawn_apple(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_apple_internal(&mut commands, &asset_server);
}

fn spawn_apple_internal(commands: &mut Commands, asset_server: &AssetServer) {
    let texture = asset_server.load("sprites/apple.png");

    let mut rng = rand::rng();
    let x = rng.random_range(-GRID_WIDTH / 2..GRID_WIDTH / 2) as f32 * GRID_SIZE;
    let y = rng.random_range(-GRID_HEIGHT / 2..GRID_HEIGHT / 2) as f32 * GRID_SIZE;

    commands.spawn((
        Sprite {
            image: texture,
            custom_size: Some(Vec2::splat(GRID_SIZE)),
            ..default()
        },
        Transform::from_translation(Vec3::new(x, y, 0.0)),
        GlobalTransform::default(),
        Apple,
    ));
}

pub fn check_apple_collision(
    mut commands: Commands,
    mut pending_tail: ResMut<PendingTail>,
    mut apple_query: Query<(Entity, &Transform), With<Apple>>,
    head_query: Query<&Transform, With<SnakeHead>>,
    asset_server: Res<AssetServer>,
) {
    let Ok(head_transform) = head_query.single() else { return };

    for (entity, transform) in &mut apple_query {
        if head_transform.translation.distance(transform.translation) < 16.0 {
            commands.entity(entity).despawn();

            pending_tail.0.push(head_transform.translation.truncate());

            spawn_apple(commands, asset_server);

            break;
        }
    }
}

