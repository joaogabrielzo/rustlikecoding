
@fragment
fn fragment(
    @location(0) world_position: vec4<f32>,
    @location(1) normals: vec3<f32>,
    @location(2) uv: vec2<f32>
) -> @location(0) vec4<f32> {
    let r = world_position.x * 0.5 + 0.5;
    let g = world_position.y * 0.5 + 0.5;
    return vec4(r, g, 0.0, 1.0);
}