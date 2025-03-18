use crate::types::SongConfig;
use bevy::prelude::*;

fn start_song(mut commands: Commands, time: Res<Time>, config: Res<SongConfig>) {
    // Song starts 3 seconds after real time
    let secs = time.elapsed_secs_f64();
    let secs_last = secs - time.delta_secs_f64();

    if secs_last <= 3. && 3. <= secs {
        commands.spawn((
            AudioPlayer::new(config.song_audio.clone()),
            PlaybackSettings::ONCE,
        ));
    }
}

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, start_song);
    }
}