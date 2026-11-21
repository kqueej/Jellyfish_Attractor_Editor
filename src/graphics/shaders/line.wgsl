struct CameraUniform {
    view_projection: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) speed: f32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) speed: f32,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    output.position =
        camera.view_projection * vec4<f32>(input.position, 1.0);

    output.speed = input.speed;

    return output;
}

fn speed_color(value: f32) -> vec3<f32> {
    let t = pow(clamp(value, 0.0, 1.0), 0.7);

    let blue   = vec3<f32>(0.13, 0.38, 1.0);
    let cyan   = vec3<f32>(0.0, 0.90, 1.0);
    let green  = vec3<f32>(0.44, 0.96, 0.42);
    let yellow = vec3<f32>(1.0, 0.90, 0.20);
    let orange = vec3<f32>(1.0, 0.44, 0.12);
    let red    = vec3<f32>(1.0, 0.13, 0.30);

    if t < 0.2 {
        return mix(blue, cyan, smoothstep(0.0, 0.2, t));
    }

    if t < 0.4 {
        return mix(cyan, green, smoothstep(0.2, 0.4, t));
    }

    if t < 0.6 {
        return mix(green, yellow, smoothstep(0.4, 0.6, t));
    }

    if t < 0.8 {
        return mix(yellow, orange, smoothstep(0.6, 0.8, t));
    }

    return mix(orange, red, smoothstep(0.8, 1.0, t));
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let color = speed_color(input.speed);

    return vec4<f32>(color, 1.0);
}
