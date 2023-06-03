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

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

const SMOOTHNESS = 0.5;

@fragment
fn fragment(
    input: VertexOutput
) -> @location(0) vec4<f32> {
    let diffuse_light_direction = vec3(0.0, 1.0, 0.0); // from above

    let directional_light = light.directional_lights[0];
    let directional_light_direction = directional_light.direction_to_light;
    let directional_light_color = directional_light.color.xyz;

    let camera_pos = vec4(0.0, 20.0, 45.0, 0.0);
    let view_direction = normalize(camera_pos - input.world_position);
    let reflect_direction = reflect(-directional_light_direction, input.world_normal);

    let texture = textureSample(tex, tex_sampler, input.uv);

    let dot_product = saturate(dot(view_direction.xyz, reflect_direction));

    let specular = vec4(pow(dot_product, SMOOTHNESS * 100.0));

    return specular * texture;
}