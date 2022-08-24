use crate::collision::Collider;
use crate::GameState;
use bevy::math::const_vec3;
use bevy::prelude::*;
use std::time::SystemTime;

const FOOD_SIZE: Vec3 = const_vec3!([30.0, 30.0, 0.0]);
const FOOD_COLOR: Color = Color::rgb(0.2, 0.7, 0.9);

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(spawn_food))
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(handle_food_consumed))
            .add_event::<FoodConsumptionEvent>();
    }
}

pub struct FoodConsumptionEvent;

// okay. uhhh. spawn first food on enter of GameState::Game?
// then when food is eaten, emit a food-eaten event
// have systems listening for event
// one system will spawn the next food
// the other system will modify snake length

#[derive(Component)]
pub struct Food;

// todo: collider: https://www.youtube.com/watch?v=WN0XK8wddac&t=465s

fn spawn_food(mut commands: Commands) {
    commands
        .spawn()
        .insert(Food)
        .insert(Collider)
        .insert_bundle(SpriteBundle {
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

fn handle_food_consumed(mut event_reader: EventReader<FoodConsumptionEvent>) {
    for food_event in event_reader.iter() {
        println!("HEYYYY WE ATE SOME FOOD!!!... {:?}", SystemTime::now());
    }
}
