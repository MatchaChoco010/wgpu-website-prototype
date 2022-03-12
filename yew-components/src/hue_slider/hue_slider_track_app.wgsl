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

fn hsv_to_gamma_rgb(hsv: vec3<f32>) -> vec3<f32> {
    let c = hsv.z * hsv.y;
    let x = c * (1.0 -  abs((hsv.x / 60.0) % 2.0 - 1.0));
    let m = hsv.z - c;
    let h = (hsv.x + 360.0) % 360.0;
    if (h < 60.0) {
        return vec3<f32>(c + m, x + m, 0.0 + m);
    } else if (h < 120.0) {
        return vec3<f32>(x + m, c + m, 0.0 + m);
    } else if (h < 180.0) {
        return vec3<f32>(0.0 + m, c + m, x + m);
    } else if (h < 240.0) {
        return vec3<f32>(0.0 + m, x + m, c + m);
    } else if (h < 300.0) {
        return vec3<f32>(x + m, 0.0 + m, c + m);
    } else {
        return vec3<f32>(c + m, 0.0 + m, x + m);
    }
}

@stage(fragment)
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = hsv_to_gamma_rgb(vec3<f32>(in.position.x * 360.0, 1.0, 1.0));
    return vec4<f32>(color, 1.0);
}
