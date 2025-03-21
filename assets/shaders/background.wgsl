struct ScreenSize {
    width: f32,
    height: f32,
}

@group(2) @binding(0)
var<uniform> screen_size: ScreenSize;

<<<<<<< HEAD
@group(2) @binding(1)
var<uniform> time: f32;

=======
>>>>>>> 191bb10 (stable after adding grdient color shader)
@group(1) @binding(0)
var texture: texture_2d<f32>;


@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    let uv = position.xy / vec2<f32>(screen_size.width, screen_size.height);
<<<<<<< HEAD
    return vec4(uv, abs(sin(time)), 1.0);
=======
    return vec4(uv, 0.0, 1.0);
>>>>>>> 191bb10 (stable after adding grdient color shader)
}