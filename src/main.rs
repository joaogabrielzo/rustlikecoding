use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_startup_system(spawn_basic_scene)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(update_graph_system)
        .run();
}

#[derive(Component)]
struct Shape;

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let resolution = 50;
    let step = 2.0 / resolution as f32;

    for i in 0..resolution {
        let x = 1.0 * (i as f32 + 0.5) * step - 1.0;
        let y = x * x * x;

        let r = (255.0 * (x * 0.5 + 0.5)).floor() as u8;
        let g = (255.0 * (y * 0.5 + 0.5)).floor() as u8;

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 * step })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb_u8(r, g, 0),
                    metallic: 0.7,
                    ..default()
                }),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            Shape,
        ));
    }

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn update_graph_system(time: Res<Time>, mut query: Query<&mut Transform, With<Shape>>) {
    let now = time.elapsed_seconds();
    for mut transform in &mut query {
        let x = transform.translation.x;
        transform.translation.y = (PI * (x + now)).sin();
    }
}
