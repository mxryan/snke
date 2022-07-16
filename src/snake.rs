use crate::{App, GameState, Plugin};
use bevy::math::const_vec3;
use bevy::prelude::*;

const SNAKE_HEAD_SIZE: Vec3 = const_vec3!([40.0, 40.0, 0.0]);
const SNAKE_BODY_SIZE: Vec3 = const_vec3!([40.0, 40.0, 0.0]);
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.3, 0.4, 0.9);
const STARTING_NUM_OF_BODY_SEGMENTS: u8 = 3;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(spawn_snake))
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(move_snake))
            .add_system_set(SystemSet::on_enter(GameState::Pause).with_system(handle_paused))
            .add_system_set(SystemSet::on_resume(GameState::Game).with_system(handle_resume));
    }
}

enum SnakeDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Component)]
struct SnakeHead {
    body_length: u8,
    speed: f32,
    direction: SnakeDirection,
}

#[derive(Component)]
struct SnakeBodySegment;

fn move_snake(
    keyboard_input: Res<Input<KeyCode>>,
    mut snake_xform_query: Query<(&mut Transform, &mut SnakeHead)>,
    time: Res<Time>,
) {
    let (mut snake_head_xform, mut snake) = snake_xform_query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::A) {
        snake.direction = SnakeDirection::LEFT;
    }
    if keyboard_input.just_pressed(KeyCode::Right) || keyboard_input.just_pressed(KeyCode::D) {
        snake.direction = SnakeDirection::RIGHT;
    }
    if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::W) {
        snake.direction = SnakeDirection::UP;
    }
    if keyboard_input.just_pressed(KeyCode::Down) || keyboard_input.just_pressed(KeyCode::S) {
        snake.direction = SnakeDirection::DOWN;
    }
    let new_translation = match snake.direction {
        SnakeDirection::UP => Vec3::new(
            snake_head_xform.translation.x,
            snake_head_xform.translation.y + snake.speed * time.delta_seconds(),
            snake_head_xform.translation.z,
        ),
        SnakeDirection::DOWN => Vec3::new(
            snake_head_xform.translation.x,
            snake_head_xform.translation.y - snake.speed * time.delta_seconds(),
            snake_head_xform.translation.z,
        ),
        SnakeDirection::LEFT => Vec3::new(
            snake_head_xform.translation.x - snake.speed * time.delta_seconds(),
            snake_head_xform.translation.y,
            snake_head_xform.translation.z,
        ),
        SnakeDirection::RIGHT => Vec3::new(
            snake_head_xform.translation.x + snake.speed * time.delta_seconds(),
            snake_head_xform.translation.y,
            snake_head_xform.translation.z,
        ),
    };

    snake_head_xform.translation = new_translation;
}

fn spawn_snake(mut commands: Commands) {
     commands
        .spawn()
        .insert(SnakeHead {
            body_length: STARTING_NUM_OF_BODY_SEGMENTS,
            speed: 150.0,
            direction: SnakeDirection::UP,
        })
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -240.0, 0.0),
                scale: SNAKE_HEAD_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            ..default()
        });
}

fn handle_paused(mut snake_xform_query: Query<&mut SnakeHead>) {
    let mut snake = snake_xform_query.single_mut();
    snake.speed = 0.0;
}

fn handle_resume(mut snake_xform_query: Query<&mut SnakeHead>) {
    let mut snake = snake_xform_query.single_mut();
    snake.speed = 150.0;
}
