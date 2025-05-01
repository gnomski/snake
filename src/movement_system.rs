use bevy::prelude::*;
use crate::snake::{SnakeHead, SnakeSpeed};
use crate::constants::*;

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Direction(pub Vec2);

pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Transform, &mut Direction), With<SnakeHead>>,
) {
    for (transform, mut direction) in &mut query {
        let pos = transform.translation;
        let in_cell_x = (pos.x % GRID_SIZE).abs() < 1.0;
        let in_cell_y = (pos.y % GRID_SIZE).abs() < 1.0;

        if in_cell_x && in_cell_y {
            if keyboard_input.pressed(KeyCode::KeyW) && **direction != Vec2::NEG_Y {
                **direction = Vec2::Y;
            } else if keyboard_input.pressed(KeyCode::KeyS) && **direction != Vec2::Y {
                **direction = Vec2::NEG_Y;
            } else if keyboard_input.pressed(KeyCode::KeyA) && **direction != Vec2::X {
                **direction = Vec2::NEG_X;
            } else if keyboard_input.pressed(KeyCode::KeyD) && **direction != Vec2::NEG_X {
                **direction = Vec2::X;
            }
        }
    }
}

pub fn move_snake(
    time: Res<Time>,
    speed: Res<SnakeSpeed>,
    mut query: Query<(&Direction, &mut Transform), With<SnakeHead>>,
) {
    
    for (direction, mut transform) in &mut query {
        transform.translation += direction.extend(0.0) * speed.0 * time.delta_secs();
    }
}