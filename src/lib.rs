use std::f32::consts::PI;

pub fn wave(x: f32, z: f32, t: f32) -> f32 {
    (PI * (x + z + t)).sin()
}

pub fn multi_wave(x: f32, z: f32, t: f32) -> f32 {
    let mut y = (PI * (x + t * 0.5)).sin();
    y += (2.0 * PI * (z + t)).sin() * 0.5;
    y += (PI * (x + z + t * 0.25)).sin();

    return y * (1.0 / 2.5);
}

pub fn ripple(x: f32, z: f32, t: f32) -> f32 {
    let d = (x * x + z * z).sqrt();
    let y = (PI * (4.0 * d - t)).sin();

    return y / (1.0 + 10.0 * d);
}
