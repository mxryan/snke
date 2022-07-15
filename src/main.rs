mod snake;
mod menu;
mod pause;

use crate::snake::SnakePlugin;
use bevy::math::const_vec3;
use bevy::prelude::*;
use crate::menu::MenuPlugin;
use crate::pause::PausePlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.95, 0.4, 0.4);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Splash,
    Menu,
    Game,
    Pause
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SnakePlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(PausePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_state(GameState::Menu)
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
