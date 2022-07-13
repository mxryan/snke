mod snake;

use bevy::math::const_vec3;
use bevy::prelude::*;
use crate::snake::SnakePlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.95, 0.4, 0.4);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Splash,
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SnakePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(spawn_food)
        .run();
}



fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}



fn spawn_food() {

}
