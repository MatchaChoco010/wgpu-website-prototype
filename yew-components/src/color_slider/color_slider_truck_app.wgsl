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
    out.position = (model.position + 1.0) / 2.0;
    return out;
}

// Fragment shader

struct UniformColorSlider {
    color_start: vec4<f32>;
    color_end: vec4<f32>;
    resolution: vec2<f32>;
    linear: u32;
};

@group(0) @binding(0)
var<uniform> uniform_buffer: UniformColorSlider;

@stage(fragment)
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let x = floor(in.position.x * uniform_buffer.resolution.x / 4.0);
    let y = floor(in.position.y * uniform_buffer.resolution.y / 4.0);
    let checker = (x + y) % 2.0;
    let background = mix(
      vec3<f32>(1.0, 1.0, 1.0),
      vec3<f32>(0.5, 0.5, 0.5),
      checker);

    var color: vec4<f32>;
    if (bool(uniform_buffer.linear)) {
        color = mix(
            uniform_buffer.color_start,
            uniform_buffer.color_end,
            in.position.x);
    } else {
        let rgb = pow(mix(
            pow(uniform_buffer.color_start.rgb, vec3<f32>(1.0 / 2.2)),
            pow(uniform_buffer.color_end.rgb, vec3<f32>(1.0 / 2.2)),
            in.position.x),
        vec3<f32>(2.2));
        let a = mix(
            uniform_buffer.color_start.a,
            uniform_buffer.color_end.a,
            in.position.x);
        color = vec4<f32>(rgb, a);
    };
    let color_with_background = mix(background, color.rgb, color.a);

    return vec4<f32>(pow(color_with_background, vec3<f32>(1.0 / 2.2)), 1.0);
}
