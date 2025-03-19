use bevy::prelude::*;
use crate::consts::*;
use crate::types::{Speed, SongConfig, Directions};
use crate::score::ScoreResource;

/// Keeps the textures and materials for Arrows
#[derive(Resource)]
struct ArrowSpriteResource {
    red_arrow: Handle<Image>,
    blue_arrow: Handle<Image>,
    green_arrow: Handle<Image>,
    border_arrow: Handle<Image>,
}

fn setup_arrow_materials(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    eprintln!("Setting up arrow materials");
    // Load texture handles
    let red_handle:  Handle<Image> = asset_server.load("images/arrow_red.png");
    let blue_handle: Handle<Image> = asset_server.load("images/arrow_blue.png");
    let green_handle: Handle<Image> = asset_server.load("images/arrow_green.png");
    let border_handle: Handle<Image> = asset_server.load("images/arrow_border.png");

    // Insert the resource into the world
    commands.insert_resource(ArrowSpriteResource {
        red_arrow: red_handle,
        blue_arrow: blue_handle,
        green_arrow: green_handle,
        border_arrow: border_handle,
    });
}

/// Arrow component
#[derive(Component)]
struct Arrow{
    speed: Speed,
    direction: Directions,
}

// #[derive(Resource)]
// struct SpawnTimer(Timer);

fn spawn_arrows(
    mut commands: Commands,
    mut song_config: ResMut<SongConfig>,
    materials: Res<ArrowSpriteResource>,
    time: Res<Time>,) 
{
    // We get the current time since startup (secs) and the time since the last iteration (secs_last),
    // this way we check if any arrows should spawn in this window

    // Song starts 3 seconds after start, so we subtract 3 seconds
    let secs = time.elapsed_secs_f64() - 3.;
    let secs_last = secs - time.delta().as_secs_f64();

    // Counter of how many arrows we need to spawn and remove from the list
    let mut remove_counter = 0;
    for arrow in &song_config.arrows {
        // List is ordered, so we can just check until an item fails
        // Check if arrow should be spawned at any point between last frame and this frame
        if secs_last < arrow.spawn_time && arrow.spawn_time < secs {
            remove_counter += 1;

            // Get the correct material according to speed
            let material = match arrow.speed {
                Speed::Slow => materials.red_arrow.clone(),
                Speed::Medium => materials.blue_arrow.clone(),
                Speed::Fast => materials.green_arrow.clone(),
            };    

            let mut transform = 
                Transform::from_translation(Vec3::new(SPAWN_POSITION, arrow.direction.y(), 1.0));
    
            transform.rotate(Quat::from_rotation_z(arrow.direction.rotation()));
    
            commands
                .spawn((
                    Sprite {
                        image: material,
                        custom_size: Some(Vec2::new(140.0, 140.0)),
                        ..Default::default()
                    },
                    transform)
                ).insert(Arrow{
                    speed: arrow.speed,
                    direction: arrow.direction,
                });  
        } else {
            break;
        }
    }

    // Remove the arrows we have spawned from the list
    for _ in 0..remove_counter {
        song_config.arrows.remove(0);
    }
}

/// Moves the arrows forward
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, arrow) in query.iter_mut() {
        transform.translation.x += time.delta_secs() * arrow.speed.value();

        let distance_after_target = transform.translation.x - (TARGET_POSITION + THRESHOLD);
        if distance_after_target >= 0.02 {
            transform.translation.y -= time.delta_secs() * distance_after_target * 2.;

            // Change the scale according to how far away the arrow is
            let scale = ((100. - distance_after_target / 3.) / 100.).max(0.2);
            transform.scale = Vec3::splat(scale);

            // Rotate the arrow according to distance and speed
            transform.rotate(Quat::from_rotation_z(
                -distance_after_target * arrow.speed.multiplier() / 460.,
            ));
        }
    }
}

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut App) {
        //app.insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
        app.add_systems(Startup, setup_arrow_materials);
        app.add_systems(Update, setup_target_arrows);
        app.add_systems(Update, spawn_arrows);
        app.add_systems(Update, move_arrows);
        app.add_systems(Update, despawn_arrows);        
    }
}

#[derive(Component)]
struct TargetArrow;

fn setup_target_arrows(mut commands: Commands, materials: Res<ArrowSpriteResource>) {
    let directions = [Directions::Up, Directions::Down, Directions::Left, Directions::Right];

    for direction in directions.iter() {
        let mut transform =
            Transform::from_translation(Vec3::new(TARGET_POSITION, direction.y(), 1.));
        transform.rotate(Quat::from_rotation_z(direction.rotation()));
        commands
            .spawn((
                Sprite {
                    image: materials.border_arrow.clone(),
                    custom_size: Some(Vec2::new(140.0, 140.0)),
                    ..Default::default()
                },
                transform))
            .insert(TargetArrow);
    }
}

/// Despawns arrows when they reach the end if the correct button is clicked
fn despawn_arrows(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Arrow)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<ScoreResource>,
) {
    for (entity, transform, arrow) in query.iter() {
        let pos = transform.translation.x;

        // Check if arrow is inside clicking threshold
        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && arrow.direction.key_just_pressed(&keyboard_input)
        {
            commands.entity(entity).despawn();

            let _points = score.increase_correct(TARGET_POSITION - pos);
        }

        // Despawn arrows after they leave the screen
        if pos >= 2. * TARGET_POSITION {
            commands.entity(entity).despawn();
            score.increase_fails();
        }
    }
}