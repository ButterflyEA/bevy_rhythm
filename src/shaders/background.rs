

use super::*;

#[derive(Component)]
pub struct Background;

pub const VERTEX_SHADER_ASSET_PATH: &str = "background.vert";
pub const FRAGMENT_SHADER_ASSET_PATH: &str = "background.frag";

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct BackgroundShader{}

impl Material2d for BackgroundShader {
    fn vertex_shader() ->ShaderRef {
        VERTEX_SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        FRAGMENT_SHADER_ASSET_PATH.into()
    }

        // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
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

    let material = materials.add(BackgroundShader{});

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_translation(Vec3::new(window_width + 10., window_height + 10., 1.0)),
        )
    ).insert(Background);

}

pub struct ShadersPlugin;
impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background);
    }
}