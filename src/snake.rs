use crate::{App, GameState, Plugin};
use bevy::math::const_vec3;
use bevy::prelude::*;

const SNAKE_HEAD_SIZE: Vec3 = const_vec3!([40.0, 40.0, 0.0]);
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.3, 0.4, 0.9);

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(spawn_snake))
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(move_snake));
    }
}

enum SnakeDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Component)]
struct Snake {
    body_length: u8,
    speed: f32,
    direction: SnakeDirection,
}

fn move_snake(
    keyboard_input: Res<Input<KeyCode>>,
    mut transform_query: Query<(&mut Transform, &mut Snake)>,
    time: Res<Time>,
) {
    let (mut snake_head_xform, mut snake) = transform_query.single_mut();
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
        .insert(Snake {
            body_length: 0,
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
