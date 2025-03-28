use super::*;

#[derive(Component)]
pub struct Background;

pub const SHADER_ASSET_PATH: &str = "shaders/background.wgsl";
<<<<<<< HEAD
pub const SHADER_ASSET_PATH_SIN_WAVE: &str = "shaders/background_sin_wave.wgsl";
=======
>>>>>>> 191bb10 (stable after adding grdient color shader)

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct BackgroundShader{
    #[uniform(0)]
    screen_size: Vec2,
<<<<<<< HEAD
    #[uniform(1)]
    time: f32,
=======
>>>>>>> 191bb10 (stable after adding grdient color shader)
}

impl Material2d for BackgroundShader {
    fn fragment_shader() -> ShaderRef {
<<<<<<< HEAD
        SHADER_ASSET_PATH_SIN_WAVE.into()
    }
}

pub fn update_time(
    mut materials: ResMut<Assets<BackgroundShader>>,
    time: Res<Time>,
    res: Query<&mut MeshMaterial2d<BackgroundShader>, With<Background>>
) {
    let time_from_start = time.elapsed_secs();
    for res_handle in res.iter() {
        let material = materials.get_mut(res_handle).unwrap();
        material.time = time_from_start;
=======
        SHADER_ASSET_PATH.into()
>>>>>>> 191bb10 (stable after adding grdient color shader)
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
<<<<<<< HEAD
        time: 0.0,
=======
>>>>>>> 191bb10 (stable after adding grdient color shader)
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
<<<<<<< HEAD
        .add_systems(Startup, setup_background)
        .add_systems(Update, update_time);
=======
        .add_systems(Startup, setup_background);
>>>>>>> 191bb10 (stable after adding grdient color shader)
    }
}