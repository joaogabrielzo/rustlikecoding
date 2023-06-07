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
var tex_normal: texture_2d<f32>;
@group(1) @binding(4)
var tex_normal_sampler: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

const BUMP_SCALE = 0.4;

@fragment
fn fragment(
    input: VertexOutput
) -> @location(0) vec4<f32> {
    let directional_light = light.directional_lights[0];
    let directional_light_direction = directional_light.direction_to_light;
    let directional_light_color = directional_light.color.xyz;

    let normal = textureSample(tex_normal, tex_normal_sampler, input.uv);
    let normal_texel_size = 1.0 / vec2<f32>(textureDimensions(tex_normal));

    let normals_values = normal.xyz * 2.0 - 1.0;
    let normals = vec3(normals_values.xy * BUMP_SCALE, normals_values.z).xzy;

    let texture = textureSample(tex, tex_sampler, input.uv);
    let main_texel_size = 1.0 / vec2<f32>(textureDimensions(tex));

    let dot_product = saturate(dot(directional_light_direction, normalize(normals)));

    let diffuse = texture * vec4(dot_product);

    return diffuse;
}