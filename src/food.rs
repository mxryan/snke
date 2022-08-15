use bevy::math::const_vec3;
use bevy::prelude::*;
use crate::GameState;

const FOOD_SIZE: Vec3 = const_vec3!([30.0, 30.0, 0.0]);
const FOOD_COLOR: Color = Color::rgb(0.2, 0.7, 0.9);


pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Game).with_system(spawn_food)
            );
    }
}

// okay. uhhh. spawn first food on enter of GameState::Game?
// then when food is eaten, emit a food-eaten event
// have systems listening for event
// one system will spawn the next food
// the other system will modify snake length

#[derive(Component)]
struct Food;


fn spawn_food(mut commands: Commands) {
    commands.spawn().insert(Food).insert_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: FOOD_SIZE,
            ..default()
        },
        sprite: Sprite {
            color: FOOD_COLOR,
            ..default()
        },
        ..default()
    });
}

