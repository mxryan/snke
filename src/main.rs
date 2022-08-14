mod audio;
mod menu;
mod pause;
mod snake;

use crate::audio::GameAudioPlugin;
use crate::menu::MenuPlugin;
use crate::pause::PausePlugin;
use crate::snake::SnakePlugin;
use bevy::math::const_vec3;
use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.95, 0.4, 0.4);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Splash,
    Menu,
    Game,
    Pause,
}

// https://mbuffett.com/posts/bevy-snake-tutorial/

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SnakePlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(PausePlugin)
        .add_plugin(GameAudioPlugin)
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
