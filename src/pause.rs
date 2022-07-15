use crate::GameState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Game).with_system(handle_pause))
            .add_system_set(SystemSet::on_update(GameState::Pause).with_system(handle_unpause));
    }
}

fn handle_pause(mut keyboard: ResMut<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        println!("pause");
        game_state.push(GameState::Pause).unwrap();
        keyboard.clear();
    }
}

fn handle_unpause(mut keyboard: ResMut<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        println!("unpause");
        game_state.pop().unwrap();
        keyboard.clear();
    }
}
