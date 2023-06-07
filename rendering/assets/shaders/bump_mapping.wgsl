#import bevy_pbr::mesh_view_bindings

@group(0) @binding(1)
var<uniform> light: Lights;

struct CustomMaterial {
    color: vec4<f32>,
}

@group(1) @binding(0)
var <uniform> material: CustomMaterial;

@group(1) @binding(1)
var tex: texture_2d<f32>;
@group(1) @binding(2)
var tex_sampler: sampler;

@group(1) @binding(3)
var tex_heights: texture_2d<f32>;
@group(1) @binding(4)
var tex_heights_sampler: sampler;

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
    let directional_light = light.directional_lights[0];
    let directional_light_direction = directional_light.direction_to_light;
    let directional_light_color = directional_light.color.xyz;

    let heights = textureSample(tex_heights, tex_heights_sampler, input.uv);
    let heights_texel_size = 1.0 / vec2<f32>(textureDimensions(tex_heights));

    let du = vec2(heights_texel_size.x * 0.5, 0.0);
    let u1 = textureSample(tex_heights, tex_heights_sampler, input.uv - du);
    let u2 = textureSample(tex_heights, tex_heights_sampler, input.uv + du);

    let dv = vec2(0.0, heights_texel_size.y * 0.5);
    let v1 = textureSample(tex_heights, tex_heights_sampler, input.uv - dv);
    let v2 = textureSample(tex_heights, tex_heights_sampler, input.uv + dv);

    // let tu = vec3(1.0, u2.x - u1.x, 0.0);
    // let tv = vec3(0.0, v2.x - v1.x, 1.0);
    // let normals = normalize(cross(tv, tu));
    let normals = normalize(vec3(u1.x - u2.x, 1.0, v1.x - v2.x));

    let texture = textureSample(tex, tex_sampler, input.uv);
    let main_texel_size = 1.0 / vec2<f32>(textureDimensions(tex));

    let dot_product = saturate(dot(directional_light_direction, normalize(normals)));

    let diffuse = vec4(dot_product);

    return diffuse;
}