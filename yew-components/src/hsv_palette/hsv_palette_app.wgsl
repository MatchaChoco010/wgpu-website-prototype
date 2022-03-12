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
    out.position = model.position;
    return out;
}

// Fragment shader

@group(0) @binding(0)
var<uniform> color: vec4<f32>;

let pi = 3.1415926536;
let resolution: vec2<f32> = vec2<f32>(128.0, 128.0);

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

fn gamma_rgb_to_hsv(rgb: vec3<f32>) -> vec3<f32> {
    let cmax = max(max(rgb.r, rgb.g), rgb.b);
    let cmin = min(min(rgb.r, rgb.g), rgb.b);
    let delta = cmax - cmin;
    var h: f32;
    if (delta == 0.0) {
        h = 0.0;
    } else if (cmax == rgb.r) {
        h = 60.0 * ((rgb.g - rgb.b) / delta % 6.0);
        h = (h + 360.0) % 360.0;
    } else if (cmax == rgb.g) {
        h = 60.0 * ((rgb.b - rgb.r) / delta + 2.0);
    } else if (cmax == rgb.b) {
        h = 60.0 * ((rgb.r - rgb.g) / delta + 4.0);
    }
    var s: f32;
    if (cmax == 0.0) {
        s = 0.0;
    } else {
        s = delta / cmax;
    }
    let v = cmax;
    return vec3<f32>(h, s, v);
}

fn linear_rgb_to_hsv(rgb: vec3<f32>) -> vec3<f32> {
    return gamma_rgb_to_hsv(pow(rgb, vec3<f32>(1.0 / 2.2)));
}

fn render_ring(position: vec2<f32>) -> vec4<f32> {
    let l = length(position);
    let mask_outer_ring = l < 0.95;
    let mask_inner_ring = l > 0.85;
    let mask = mask_outer_ring && mask_inner_ring;

    let rad = atan2(position.y, position.x);
    let deg = (-rad * 180.0 / pi - 90.0 + 360.0) % 360.0;
    let c = hsv_to_gamma_rgb(vec3<f32>(deg, 1.0, 1.0));

    if (mask) {
        return vec4<f32>(c, 1.0);
    } else {
        return vec4<f32>();
    }
}

fn render_square(position: vec2<f32>) -> vec4<f32> {
    let a = abs(position);
    let mask = a.x < 0.8 / sqrt(2.0) && a.y < 0.8 / sqrt(2.0);

    let h = linear_rgb_to_hsv(color.rgb).x;
    let s = (position.x / (0.8 / sqrt(2.0)) + 1.0) / 2.0;
    let v = (position.y / (0.8 / sqrt(2.0)) + 1.0) / 2.0;
    let c = hsv_to_gamma_rgb(vec3<f32>(h, s, v));

    if (mask) {
        return vec4<f32>(c, 1.0);
    } else {
        return vec4<f32>();
    }
}

fn render(position: vec2<f32>) -> vec4<f32> {
    let ring = render_ring(position);
    let square = render_square(position);
    return mix(ring, square, square.a);
}

@stage(fragment)
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel = vec2<f32>(2.0, 2.0) / resolution;
    var color = vec4<f32>();
    color = color
        + render(in.position + vec2<f32>(-pixel.x / 8.0, -pixel.y * 3.0 / 8.0));
    color = color
        + render(in.position + vec2<f32>(pixel.x * 3.0 / 8.0, -pixel.y / 8.0));
    color = color
        + render(in.position + vec2<f32>(pixel.x / 8.0, pixel.y * 3.0 / 8.0));
    color = color
        + render(in.position + vec2<f32>(-pixel.x * 3.0 / 8.0, pixel.y / 8.0));
    color = color / 4.0;
    return vec4<f32>(color.rgb * color.a, color.a);
}
