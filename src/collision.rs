use crate::GameState;
use bevy::prelude::*;
use crate::food::{Food, FoodConsumptionEvent};
use crate::snake::SnakeHead;
use bevy::sprite::collide_aabb;
use bevy::sprite::collide_aabb::Collision;

#[derive(Component)]
pub struct Collider;

#[derive(Default)]
struct CollisionEvent;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Game)
            .with_system(detect_collision).with_system(detect_collision));
    }
}

fn detect_collision(
    snake_head_xform_query: Query<&Transform, (With<Collider>, With<SnakeHead>)>,
    food_xform_query: Query<&Transform, (With<Collider>, With<Food>)>,
    mut event_writer: EventWriter<FoodConsumptionEvent>
) {
    let snake_head_xf = snake_head_xform_query.single();
    let food_xform = food_xform_query.single();
    let collision = collide_aabb::collide(
        snake_head_xf.translation,
        snake_head_xf.scale.truncate(),
        food_xform.translation,
        food_xform.scale.truncate()
    );
    if collision.is_some() {
        event_writer.send(FoodConsumptionEvent);
    }
}
