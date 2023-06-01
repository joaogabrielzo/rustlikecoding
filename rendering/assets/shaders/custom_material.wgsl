struct CustomMaterial {
    color: vec4<f32>,
}

@group(1) @binding(0)
var <uniform> material: CustomMaterial;

@group(1) @binding(1)
var grid_texture: texture_2d<f32>;
@group(1) @binding(2)
var grid_sampler: sampler;

@group(1) @binding(3)
var grid_detail_texture: texture_2d<f32>;
@group(1) @binding(4)
var grid_detail_sampler: sampler;

@group(1) @binding(5)
var marble_texture: texture_2d<f32>;
@group(1) @binding(6)
var marble_sampler: sampler;

@group(1) @binding(7)
var marble_detail_texture: texture_2d<f32>;
@group(1) @binding(8)
var marble_detail_sampler: sampler;

@group(1) @binding(9)
var splat_map_texture: texture_2d<f32>;
@group(1) @binding(10)
var splat_map_sampler: sampler;

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
    let grid_detail_tex = textureSample(grid_detail_texture, grid_detail_sampler, input.uv * 10.0);
    var grid_tex = textureSample(grid_texture, grid_sampler, input.uv * 10.0);

    let marble_detail_tex = textureSample(marble_detail_texture, marble_detail_sampler, input.uv * 10.0);
    var marble_tex = textureSample(marble_texture, marble_sampler, input.uv * 10.0);

    let splat_map = textureSample(splat_map_texture, splat_map_sampler, input.uv);

    return grid_tex * splat_map.x + marble_tex * (1.0 - splat_map.x);
}