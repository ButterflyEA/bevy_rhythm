use bevy::prelude::*;
use crate::score::ScoreResource;

#[derive(Component)]
struct TimeText;

fn setup_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        // Time text node
        .spawn( (
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.),
                top: Val::Px(10.),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            }, 
            BackgroundColor(Color::NONE)
        ))
        .with_children(|parent| {
            // Spawn the text child
            parent
                .spawn((
                    Text::new("Time: 0.0"),
                    TextFont {
                        font: font.clone(),
                        font_size: 40.0,
                        ..Default::default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9))
                    ), // Replaces TextBundle
                )
                .insert(TimeText);
        });

        commands
        // Score text node
        .spawn( (
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.),
                bottom: Val::Px(10.),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            }, 
            BackgroundColor(Color::NONE)
        ))
        .with_children(|parent| {
            // Spawn the text child
            parent
                .spawn((
                    Text::new("Score: 0. Corrects: 0. Fails: 0"),
                    TextFont {
                        font: font.clone(),
                        font_size: 40.0,
                        ..Default::default()
                    },
                    TextColor(Color::srgb(0.78, 0.62, 0.192))
                    ), // Replaces TextBundle
                )
                .insert(ScoreText);
        });
}

fn update_time_text(time: Res<Time>, mut query: Query<(&mut Text, &TimeText)>) {
    // Song starts 3 seconds after real time
    let secs = time.elapsed_secs_f64() - 3.;

    // Don't do anything before the song starts
    if secs < 0. {
        return;
    }

    for (mut text, _marker) in query.iter_mut() {
        **text = format!("Time: {:.2}", secs);
    }
}

#[derive(Component)]
struct ScoreText;

fn update_score_text(score: ResMut<ScoreResource>, mut query: Query<(&mut Text, &ScoreText)>) {
    for (mut text, _marker) in query.iter_mut() {
        **text = format!(
            "Score: {}. Corrects: {}. Fails: {}",
            score.score(),
            score.corrects(),
            score.fails()
        );
    }
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ui)
            .add_systems(Update, update_time_text)
            .add_systems(Update, update_score_text);
    }
}