use super::*;

#[derive(Component)]
pub struct TargetArrowSparkle {
    direction: Directions,
}

#[derive(Component)]
pub struct ShaderInputs {
    time: f32,
    resolution: Vec2,
}

#[derive(Resource, Reflect)]
pub struct TimeSinceLastCorrect {
    last_time: f32,
    points: f32,
}

impl Default for TimeSinceLastCorrect {
    fn default() -> Self {
        Self {
            last_time: 0.0,
            points: 0.0,
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct TargetArrowsShader{
    #[uniform(0)]
    time_since_last_correct: f32,
    #[uniform(1)]
    points_since_last_correct: f32,
}

impl Material2d for TargetArrowsShader {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

pub const SHADER_ASSET_PATH: &str = "shaders/target_arrows.wgsl";

pub fn setup_target_arrows(
    mut commands: Commands,
    mut shaders: ResMut<Assets<Shader>>,
    mut materials: ResMut<Assets<TargetArrowsShader>>,
    windows: Query<&Window>,
) {
    // Get the primary window
    let window = windows.single();

    let directions = [Directions::Up, Directions::Down, Directions::Left, Directions::Right];
    for direction in directions.iter() {
        // Different z values so they don't overlap
        let z = match direction {
            Directions::Up => 0.3,
            Directions::Down => 0.4,
            Directions::Left => 0.5,
            Directions::Right => 0.6,
        };

        let mut transform =
            Transform::from_translation(Vec3::new(TARGET_POSITION, direction.y(), z));
        transform.scale = Vec3::new(300., 300., 1.);

        commands
            .spawn(
                (MeshMaterial2d(materials.add(TargetArrowsShader{
                    time_since_last_correct: 3.0,
                    points_since_last_correct: 0.5,
                })),
                transform)
            )
            .insert(TargetArrowSparkle {
                direction: *direction,
            })
            .insert(ShaderInputs {
                time: 0.,
                resolution: Vec2::new(window.width() / window.height(), 1.),
            });
    }
}

