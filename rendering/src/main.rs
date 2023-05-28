mod transformations;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use transformations::*;

fn main() {
    App::new()
        .register_type::<Translation>()
        .register_type::<Scaling>()
        .register_type::<Rotation>()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_system(spawn_basic_scene)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Like Coding".into(),
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::defaultMathFunction())
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(update_transformations_system)
        .run();
}

const GRID_RESOLUTION: i32 = 10;

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Translation::default());
    commands.spawn(Scaling::default());
    commands.spawn(Rotation::default());

    for z in 0..GRID_RESOLUTION {
        for y in 0..GRID_RESOLUTION {
            for x in 0..GRID_RESOLUTION {
                commands.spawn((
                    MaterialMeshBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                        transform: Transform::from_translation(get_coordinates(x, y, z)),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(
                                x as f32 / GRID_RESOLUTION as f32,
                                y as f32 / GRID_RESOLUTION as f32,
                                z as f32 / GRID_RESOLUTION as f32,
                            ),
                            ..default()
                        }),
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    Shape,
                ));
            }
        }
    }

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });

    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..default()
    // });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn get_coordinates(x: i32, y: i32, z: i32) -> Vec3 {
    return Vec3 {
        x: x as f32 - (GRID_RESOLUTION - 1) as f32 * 0.5,
        y: y as f32 - (GRID_RESOLUTION - 1) as f32 * 0.5,
        z: z as f32 - (GRID_RESOLUTION - 1) as f32 * 0.5,
    };
}

#[derive(Component)]
struct Shape;

fn update_transformations_system(
    _translation_query: Query<&Translation, Changed<Translation>>,
    _scaling_query: Query<&Scaling, Changed<Scaling>>,
    rotation_query: Query<&Rotation, Changed<Rotation>>,
    mut transform_query: Query<&mut Transform, With<Shape>>,
) {
    if let Ok(rotation) = rotation_query.get_single() {
        for mut transform in &mut transform_query {
            let rad_z = rotation.v.z.to_radians();
            let sin_z = rad_z.sin();
            let cos_z = rad_z.cos();

            transform.rotation = Quat::from_xyzw(
                transform.rotation.x * cos_z - transform.rotation.y * sin_z,
                transform.rotation.x * sin_z - transform.rotation.y * cos_z,
                rad_z,
                1.0,
            );
        }
    }
}
