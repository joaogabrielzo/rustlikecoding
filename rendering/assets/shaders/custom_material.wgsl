struct CustomMaterial {
    color: vec4<f32>,
}

@group(1) @binding(0)
var <uniform> material: CustomMaterial;

@group(1) @binding(1)
var base_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_sampler: sampler;

@group(1) @binding(3)
var detail_texture: texture_2d<f32>;
@group(1) @binding(4)
var detail_sampler: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@fragment
fn fragment(
    input: VertexOutput
) -> @location(0) vec4<f32> {
    let detail_tex = textureSample(detail_texture, detail_sampler, input.uv * 10.0);

    var tex = textureSample(base_texture, base_sampler, input.uv);
    tex *= tex * detail_tex;

    return tex;
}