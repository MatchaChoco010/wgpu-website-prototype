// Vertex shader

struct VertexInput {
    @location(0) position: vec2<f32>;
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>;
    @location(0) position: vec2<f32>;
};

@stage(vertex)
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
    out.position = (model.position + vec2<f32>(1.0, 1.0)) / 2.0;
    return out;
}

// Fragment shader

@group(0) @binding(0)
var<uniform> color: vec4<f32>;

@stage(fragment)
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let x = floor(in.position.x * 8.0);
    let y = floor(in.position.y * 4.0);
    let checker = (x + y) % 2.0;
    let background = mix(
      vec4<f32>(1.0, 1.0, 1.0, 1.0),
      vec4<f32>(0.5, 0.5, 0.5, 1.0),
      checker);

    let w_alpha = mix(color.rgb, background.rgb, color.a);
    let wo_alpha = color.rgb;

    let t = floor(in.position.x * 2.0);
    let color = mix(wo_alpha, w_alpha, t);

    return vec4<f32>(pow(color, vec3<f32>(1.0 / 2.2)), 1.0);
}
