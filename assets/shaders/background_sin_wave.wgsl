#define TWO_PI 6.28318530718

struct ScreenSize {
    width: f32,
    height: f32,
}

@group(2) @binding(0)
var<uniform> screen_size: ScreenSize;

@group(2) @binding(1)
var<uniform> time: f32;

@group(1) @binding(0)
var texture: texture_2d<f32>;

// Custom mod function to mimic GLSL’s mod, vectorized
fn mod_glsl(x: vec3<f32>, y: f32) -> vec3<f32> {
    return x - y * floor(x / y);
}

// HSB to RGB conversion with corrected types
fn hsb2rgb(c: vec3<f32>) -> vec3<f32> {
    let rgb = clamp(abs(mod_glsl(c.x * 6.0 + vec3<f32>(0.0, 4.0, 2.0), 6.0) - 3.0) - 1.0, vec3<f32>(0.0), vec3<f32>(1.0));
    let rgb_squared = rgb * rgb;
    return c.z * mix(vec3<f32>(1.0), rgb_squared * (3.0 - 2.0 * rgb), c.y);
}

// Sine wave function
fn wave_sin(x: f32) -> f32 {
    let amplitude = 0.5;
    let frequency = 1.0;
    var y = sin(x * frequency);
    let t = 0.01 * (-time * 50.0);
    y = y + sin(x * frequency * 2.1 + t) * 4.5;
    y = y + sin(x * frequency * 1.72 + t * 1.121) * 4.0;
    y = y + sin(x * frequency * 2.221 + t * 0.437) * 5.0;
    y = y + sin(x * frequency * 3.1122 + t * 4.269) * 2.5;
    y = y * amplitude * 0.06;
    return y;
}

// Cosine wave function
fn wave_cos(x: f32) -> f32 {
    let amplitude = 0.5;
    let frequency = 2.0;
    var y = cos(x * frequency);
    let t = 0.01 * (-time * 30.0);
    y = y + cos(x * frequency * 2.1 + t) * 4.5;
    y = y + cos(x * frequency * 1.72 + t * 1.121) * 4.0;
    y = y + cos(x * frequency * 2.221 + t * 0.437) * 5.0;
    y = y + cos(x * frequency * 3.1122 + t * 4.269) * 2.5;
    y = y * amplitude * 0.06;
    return y;
}

// Wave distortion function
fn wave(v: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(wave_sin(v.x), wave_cos(v.y));
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    @location(1) v_uv: vec2<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    //let uv = position.xy / vec2<f32>(screen_size.width, screen_size.height);
    let uv = wave(v_uv);
    //return vec4<f32>(uv.x, uv.y, 0.0, 1.0);
    let color = hsb2rgb(vec3<f32>(uv.x + sin(uv.y), 0.7, 1.0));
    return vec4(color, 1.0);
}