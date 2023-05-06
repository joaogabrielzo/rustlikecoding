use std::f32::consts::PI;

pub fn wave(x: f32, t: f32) -> f32 {
    (PI * (x + t)).sin()
}

pub fn multi_wave(x: f32, t: f32) -> f32 {
    let mut y = (PI * (x + t * 0.5)).sin();
    y += (2.0 * PI * (x + t)).sin() * (1.0 / 2.0);

    return y * (2.0 / 3.0);
}

pub fn ripple(x: f32, t: f32) -> f32 {
    let d = x.abs();
    let y = (PI * (4.0 * d - t)).sin();

    return y / (1.0 + 10.0 * d);
}
