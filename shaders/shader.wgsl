struct CameraUniform{
    view_proj: mat4x4<f32>
};

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput{
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec3<f32>
}

struct VertexOutput{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec3<f32>
}


@vertex

fn vs_main(model: VertexInput) -> VertexOutput{
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.color = model.color;
    out.clip_position = camera.view_proj * vec4<f32>(model.position * 0.3 + vec3(0.3, 0.1, 0.0), 1.0);
    return out;
}

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var sample: sampler;

@fragment

fn fs_main(in: VertexOutput) -> @location(0) vec4<f32>{
    return textureSample(texture, sample, in.tex_coords);
}