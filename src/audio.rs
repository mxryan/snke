use bevy::prelude::*;
use bevy_kira_audio;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioPlugin};


pub struct GameAudioPlugin;

pub struct Background;

pub struct AudioState {
    bgm_handle: Handle<AudioSource>,
    bgm_channel: AudioChannel<Background>
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_audio_channel::<Background>()
            .add_startup_system(play_bgm);
    }
}


fn play_bgm(background: Res<AudioChannel<Background>>, asset_server: Res<AssetServer>) {
    background.play(asset_server.load("audio/field_theme_2.wav"));
}