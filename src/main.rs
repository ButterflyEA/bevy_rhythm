
mod arrows;
mod consts;
mod types;
mod ui;
mod score;
mod audio;
mod shaders;


use bevy::prelude::*;
use arrows::ArrowsPlugin;
use ui::UIPlugin;
use score::ScoreResource;
use audio::AudioPlugin;
use shaders::background::ShadersPlugin;


fn main() {
    App::new()
        .insert_resource(ScoreResource::new())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rhythm".to_string(),
                resolution: (800.0, 600.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(ArrowsPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(ShadersPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, types::exit_on_esc) // System to handle ESC key
        .run();
}



fn setup(mut commands: Commands , asset_server: Res<AssetServer>) {
    let config = types::load_config("test.toml", &asset_server);
    println!("Song name: {}", config.name);
    // Spawn a camera (required to see anything in 2D)
    commands.spawn(Camera2d::default());
    commands.insert_resource(config);
        
}


