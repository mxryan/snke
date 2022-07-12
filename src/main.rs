use crate::SnakeDirection::{LEFT, UP};
use bevy::math::const_vec3;
use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.95, 0.4, 0.4);
const SNAKE_HEAD_SIZE: Vec3 = const_vec3!([40.0, 40.0, 0.0]);
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.3, 0.4, 0.9);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Splash,
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(move_snake)
        .add_system(spawn_food)
        .run();
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn()
        .insert(Snake {
            body_length: 0,
            speed: 150.0,
            direction: UP,
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

fn spawn_food() {

}
