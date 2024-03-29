use crate::collision::Collider;
use crate::{App, GameState, Plugin};
use bevy::math::const_vec3;
use bevy::prelude::*;
use std::collections::VecDeque;

const SNAKE_SQUARE_SIDE_LEN: f32 = 30.0;
const SNAKE_HEAD_SIZE: Vec3 = const_vec3!([SNAKE_SQUARE_SIDE_LEN, SNAKE_SQUARE_SIDE_LEN, 0.0]);
const SNAKE_SEGMENT_SIZE: Vec3 = const_vec3!([SNAKE_SQUARE_SIDE_LEN, SNAKE_SQUARE_SIDE_LEN, 0.0]);
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.3, 0.4, 0.9);
const STARTING_NUM_OF_BODY_SEGMENTS: u32 = 320;
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SnakeSegments::default())
            .insert_resource(DirectionQueue::default())
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(spawn_snake))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(move_snake.label(SystemLabel::HandleSnakeMovement))
                    .with_system(enqueue_direction.before(SystemLabel::HandleSnakeMovement)),
            )
            .add_system_set(SystemSet::on_enter(GameState::Pause).with_system(handle_paused))
            .add_system_set(SystemSet::on_resume(GameState::Game).with_system(handle_resume));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
enum SystemLabel {
    EnqueueMovement,
    HandleSnakeMovement,
}

#[derive(Copy, Clone, PartialEq)]
enum SnakeDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Component)]
pub struct SnakeHead {
    body_length: u32,
    speed: f32,
    direction: SnakeDirection,
    distance_traveled_since_last_dir_change: f32,
}

#[derive(Component)]
struct SnakeBodySegment;

#[derive(Default)]
struct SnakeSegments(Vec<Entity>);

#[derive(Default)]
struct DirectionQueue(VecDeque<SnakeDirection>);

// fixme: hey theres a bug. just mash the wasd keys and observe the resulting
//  movement. its not good
fn enqueue_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut direction_queue: ResMut<DirectionQueue>,
    mut snake: Query<&mut SnakeHead>,
) {
    let direction = snake.single_mut().direction;
    // fixme: actually i think this is wrong, gotta look at the last thing in the
    //  queue not the direction on the snake component
    if keyboard_input.just_pressed(KeyCode::A) && direction != SnakeDirection::RIGHT {
        direction_queue.0.push_back(SnakeDirection::LEFT);
    }
    if keyboard_input.just_pressed(KeyCode::D) && direction != SnakeDirection::LEFT {
        direction_queue.0.push_back(SnakeDirection::RIGHT);
    }
    if keyboard_input.just_pressed(KeyCode::W) && direction != SnakeDirection::DOWN {
        direction_queue.0.push_back(SnakeDirection::UP);
    }
    if keyboard_input.just_pressed(KeyCode::S) && direction != SnakeDirection::UP {
        direction_queue.0.push_back(SnakeDirection::DOWN);
    }
}

// todo make snake boduy segs overlap less
fn move_snake(
    mut direction_queue: ResMut<DirectionQueue>,
    mut snake_xform_query: Query<(&mut Transform, &mut SnakeHead)>,
    time: Res<Time>,
    mut snake_segments: ResMut<SnakeSegments>,
    mut snake_body_seg_xforms: Query<(&mut Transform, &SnakeBodySegment), Without<SnakeHead>>,
) {
    let (mut snake_head_xform, mut snake) = snake_xform_query.single_mut();

    let last_direction = snake.direction;
    if direction_queue.0.len() > 0
        && snake.distance_traveled_since_last_dir_change > SNAKE_SQUARE_SIDE_LEN
    {
        snake.direction = direction_queue.0.pop_front().unwrap();
    }

    let delta_position = snake.speed * time.delta_seconds();
    if snake.direction == last_direction {
        snake.distance_traveled_since_last_dir_change += delta_position;
    } else {
        snake.distance_traveled_since_last_dir_change = 0.0;
    }
    let new_translation = match snake.direction {
        SnakeDirection::UP => Vec3::new(
            snake_head_xform.translation.x,
            snake_head_xform.translation.y + delta_position,
            snake_head_xform.translation.z,
        ),
        SnakeDirection::DOWN => Vec3::new(
            snake_head_xform.translation.x,
            snake_head_xform.translation.y - delta_position,
            snake_head_xform.translation.z,
        ),
        SnakeDirection::LEFT => Vec3::new(
            snake_head_xform.translation.x - delta_position,
            snake_head_xform.translation.y,
            snake_head_xform.translation.z,
        ),
        SnakeDirection::RIGHT => Vec3::new(
            snake_head_xform.translation.x + delta_position,
            snake_head_xform.translation.y,
            snake_head_xform.translation.z,
        ),
    };

    let mut last_seg_pos = snake_head_xform.translation;

    snake_head_xform.translation = new_translation;

    let mut curr_seg_pos = Vec3::new(0.0, 0.0, 0.0);

    for snake_seg_entity in snake_segments.0.iter() {
        if let Ok((mut snake_body_seg_xform, _)) = snake_body_seg_xforms.get_mut(*snake_seg_entity)
        {
            curr_seg_pos = snake_body_seg_xform.translation;
            snake_body_seg_xform.translation = last_seg_pos;
            last_seg_pos = curr_seg_pos;
        }
    }
}

fn spawn_snake(mut commands: Commands, mut snake_segments: ResMut<SnakeSegments>) {
    let mut x = 0.0;
    let mut y = -100.0;
    snake_segments.0.push(
        commands
            .spawn()
            .insert(SnakeHead {
                body_length: STARTING_NUM_OF_BODY_SEGMENTS,
                speed: 150.0,
                direction: SnakeDirection::UP,
                distance_traveled_since_last_dir_change: 0.0,
            })
            .insert_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    scale: SNAKE_HEAD_SIZE,
                    ..default()
                },
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(Collider)
            .id(),
    );

    for _ in 0..STARTING_NUM_OF_BODY_SEGMENTS {
        snake_segments
            .0
            .push(spawn_segment(&mut commands, 1.0, 1.0))
    }
}

fn spawn_segment(commands: &mut Commands, x: f32, y: f32) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x, y, 0.0),
                scale: SNAKE_SEGMENT_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeBodySegment)
        .id()
}

fn handle_paused(mut snake_xform_query: Query<&mut SnakeHead>) {
    let mut snake = snake_xform_query.single_mut();
    snake.speed = 0.0;
}

fn handle_resume(mut snake_xform_query: Query<&mut SnakeHead>) {
    let mut snake = snake_xform_query.single_mut();
    snake.speed = 150.0;
}
