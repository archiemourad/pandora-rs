struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
}

@group(1) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct InstanceInput {
    @location(5) model0: vec4<f32>,
    @location(6) model1: vec4<f32>,
    @location(7) model2: vec4<f32>,
    @location(8) model3: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    let model = mat4x4<f32>(
        instance.model0,
        instance.model1,
        instance.model2,
        instance.model3,
    );

    out.tex_coords = vertex.tex_coords;
    out.clip_position = camera.view_proj * model * vec4<f32>(vertex.position, 1.0);
    
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
