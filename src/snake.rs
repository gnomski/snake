use crate::movement_system::Direction;
use crate::constants::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeBody;

#[derive(Resource, Default)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Resource, Default)]
pub struct LastPositions(pub Vec<Vec2>);

#[derive(Resource, Default)]
pub struct PendingTail(pub Vec<Vec2>);

#[derive(Resource)]
pub struct SnakeSpeed(pub f32);

pub fn spawn_snake_head(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut snake_segments: ResMut<SnakeSegments>,
    mut last_positions: ResMut<LastPositions>,
) {
    let head_entity = commands
        .spawn((
            Sprite {
                image: asset_server.load("sprites/snake_head.png"),
                custom_size: Some(Vec2::new(32., 32.)),
                ..default()
            },
            Transform::default(),
            GlobalTransform::default(),
            SnakeHead,
            Direction(Vec2::Y),
        ))
        .id();
    snake_segments.0.push(head_entity);
    let initial_pos = Vec2::ZERO;
    last_positions.0.push(initial_pos);
    last_positions.0.push(initial_pos);
}

pub fn track_head_positions(
    mut last_positions: ResMut<LastPositions>,
    segments: Res<SnakeSegments>,
    query: Query<&Transform>,
) {
    let head_transform = query.get(segments.0[0]).unwrap();
    let mut new_head_pos = head_transform.translation.truncate();

    // Приводим к сетке:
    new_head_pos.x = (new_head_pos.x / GRID_SIZE).round() * GRID_SIZE;
    new_head_pos.y = (new_head_pos.y / GRID_SIZE).round() * GRID_SIZE;

    // Если голова сместилась на новую клетку — записываем
    if last_positions.0.is_empty() || last_positions.0[0] != new_head_pos {
        last_positions.0.insert(0, new_head_pos);
    }

    // Догоняем историю до количества сегментов
    if let Some(last) = last_positions.0.last().copied() {
        while last_positions.0.len() < segments.0.len() {
            last_positions.0.push(last);
        }
    }

    // Подрезаем лишнее
    if last_positions.0.len() > segments.0.len() {
        last_positions.0.pop();
    }
}

pub fn update_body_positions(
    last_positions: Res<LastPositions>,
    segments: Res<SnakeSegments>,
    mut query: Query<&mut Transform>,
) {
     /*    // Логируем last_positions
        let positions: Vec<String> = last_positions
        .0
        .iter()
        .enumerate()
        .map(|(i, pos)| format!("[{}] ({:.1}, {:.1})", i, pos.x, pos.y))
        .collect();
    info!("📦 last_positions: {}", positions.join(", "));

    // Логируем entity IDs в snake_segments
    let segment_ids: Vec<String> = segments
        .0
        .iter()
        .enumerate()
        .map(|(i, entity)| format!("[{}] {:?}", i, entity))
        .collect();
    info!("📦 snake_segments: {}", segment_ids.join(", "));

    // Логируем transform каждого сегмента
    for (i, &entity) in segments.0.iter().enumerate().skip(1) {
        if let Some(pos) = last_positions.0.get(i) {
            if let Ok(transform) = query.get(entity) {
                info!(
                    "🔧 Segment {} (entity {:?}) → current ({:.1}, {:.1}), target ({:.1}, {:.1})",
                    i,
                    entity,
                    transform.translation.x,
                    transform.translation.y,
                    pos.x,
                    pos.y
                );
            }
        }
    }*/
    if last_positions.0.len() < segments.0.len() {
        return; // защита от рассинхрона
    }

    for (i, &entity) in segments.0.iter().enumerate().skip(1) {
        if let Some(pos) = last_positions.0.get(i) {
            if let Ok(mut transform) = query.get_mut(entity) {
                transform.translation = pos.extend(0.0);
            }
        }
    }
}
pub fn process_tail(
    mut commands: Commands,
    mut pending_tail: ResMut<PendingTail>,
    mut snake_segments: ResMut<SnakeSegments>,
    mut last_positions: ResMut<LastPositions>,
    head_query: Query<&Transform, With<SnakeHead>>,
    asset_server: Res<AssetServer>,
) {
    let Ok(head_transform) = head_query.single() else { return };
    let head_pos = head_transform.translation.truncate();

    pending_tail.0.retain(|&pos| {
        if head_pos.distance(pos) >= GRID_SIZE - 0.1 {
            info!(
                "🐍 Спавним хвост: голова сейчас: ({:.1}, {:.1}), хвост на: ({:.1}, {:.1})",
                head_pos.x, head_pos.y, pos.x, pos.y
            );

            let body_entity = commands.spawn((
                Sprite {
                    image: asset_server.load("sprites/snake_body.png"),
                    custom_size: Some(Vec2::new(GRID_SIZE, GRID_SIZE)),
                    ..default()
                },
                Transform::from_translation(pos.extend(0.0)),
                GlobalTransform::default(),
                SnakeBody,
            )).id();

            snake_segments.0.push(body_entity);

           // let last_tail_pos = *last_positions.0.last().unwrap();
            last_positions.0.push(pos);

            false // удаляем из pending_tail, задача выполнена
        } else {
            true // оставляем до следующего кадра
        }
    });
}
