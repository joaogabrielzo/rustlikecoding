use std::f32::consts::PI;

use bevy::prelude::Vec3;

pub type MathFunction = fn(f32, f32, f32) -> Vec3;

pub fn wave(u: f32, v: f32, t: f32) -> Vec3 {
    let x = u;
    let y = (PI * (u + v + t)).sin();
    let z = v;

    Vec3 { x, y, z }
}

pub fn multi_wave(u: f32, v: f32, t: f32) -> Vec3 {
    let x = u;
    let mut y = (PI * (u + t * 0.5)).sin();
    y += (2.0 * PI * (v + t)).sin() * 0.5;
    y += (PI * (u + v + t * 0.25)).sin();
    y *= 1.0 / 2.5;
    let z = v;

    return Vec3 { x, y, z };
}

pub fn ripple(u: f32, v: f32, t: f32) -> Vec3 {
    let d = (u * u + v * v).sqrt();

    let x = u;
    let mut y = (PI * (4.0 * d - t)).sin();
    y /= 1.0 + 10.0 * d;
    let z = v;

    return Vec3 { x, y, z };
}

pub fn sphere(u: f32, v: f32, t: f32) -> Vec3 {
    let r = 0.9 + 0.1 * (PI * (6.0 * u + 4.0 * v + t)).sin();
    let s = r * (PI * 0.5 * v).cos();

    let x = s * (PI * u).sin();
    let y = r * (PI * 0.5 * v).sin();
    let z = s * (PI * u).cos();

    return Vec3 { x, y, z };
}

pub fn torus(u: f32, v: f32, t: f32) -> Vec3 {
    let r1 = 0.7 + 0.1 * (PI * (6.0 * u + 0.5 * t)).sin();
    let r2 = 0.15 + 0.05 * (PI * (8.0 * u + 4.0 * v + 2.0 * t)).sin();
    let s = r1 + r2 * (PI * v).cos();

    let x = s * (PI * u).sin();
    let y = r2 * (PI * v).sin();
    let z = s * (PI * u).cos();

    return Vec3 { x, y, z };
}

pub fn morph(u: f32, v: f32, t: f32, from: MathFunction, to: MathFunction, progress: f32) -> Vec3 {
    let lhs = from(u, v, t);
    let rhs = to(u, v, t);

    return lhs.lerp(rhs, smoothstep(0.0, 1.0, progress))
}

fn smoothstep(low: f32, high: f32, x: f32) -> f32 {
    let x = clamp((x - low) / (high - low));

    return x * x * (3.0 - 2.0 * x);
}

fn clamp(x: f32) -> f32 {
    if x > 1.0 {
        return 1.0;
    } else if x < 0.0 {
        return 0.0;
    }

    return x;
}