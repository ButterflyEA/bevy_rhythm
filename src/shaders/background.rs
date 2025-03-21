use super::*;

#[derive(Component)]
pub struct Background;

pub const SHADER_ASSET_PATH: &str = "shaders/background.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct BackgroundShader{
    #[uniform(0)]
    screen_size: Vec2,
}

impl Material2d for BackgroundShader {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

pub fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundShader>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let window_width = window.width();
    let window_height = window.height();

    let mesh = meshes.add(
        Mesh::from(Rectangle::new(window_width, window_height),
));

    let material = materials.add(BackgroundShader{
        screen_size: Vec2::new(window_width, window_height),
    });

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_scale(Vec3::new(window_width, window_height, -1.0)),
        )
    ).insert(Background);

}

pub struct ShadersPlugin;
impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(Material2dPlugin::<BackgroundShader>::default())
        .add_systems(Startup, setup_background);
    }
}