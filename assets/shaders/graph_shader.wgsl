
@fragment
fn fragment(
    @builtin(position) coord: vec4<f32>
) -> @location(0) vec4<f32> {
    return vec4(coord);
}