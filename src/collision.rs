use crate::GameState;
use bevy::prelude::*;

#[derive(Component)]
pub struct Collider;

#[derive(Default)]
struct CollisionEvent;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(detect_collison));
    }
}

fn detect_collison(collidables: Query<Entity, With<Collider>>) {
    let mut count = 0;

    println!("LOOKING FOR COLLISIONS... num of collidables... {}", count);
}
