use crate::game::SnakeHead;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Apple;

const GRID_SIZE: f32 = 32.0;
const GRID_WIDTH: i32 = 20; // поле будет от -10 до +10
const GRID_HEIGHT: i32 = 15;

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
    mut apple_query: Query<(Entity, &Transform), With<Apple>>,
    head_query: Query<&Transform, With<SnakeHead>>,
    asset_server: Res<AssetServer>,
) {
    let Ok(head_transform) = head_query.single() else {
        return;
    };

    let mut apple_to_despawn: Option<Entity> = None;

    for (entity, transform) in &mut apple_query {
        if head_transform.translation.distance(transform.translation) < 16.0 {
            apple_to_despawn = Some(entity);
            break;
        }
    }

    if let Some(entity) = apple_to_despawn {
        commands.entity(entity).despawn();

        // Теперь можно безопасно передавать команды дальше
        spawn_apple(commands, asset_server);
    }
}
