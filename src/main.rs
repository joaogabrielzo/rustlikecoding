use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::window::PresentMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_startup_system(spawn_basic_scene)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Like Coding".into(),
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(MaterialPlugin::<GraphMaterial>::default())
        .add_system(update_graph_system)
        .run();
}

#[derive(Component)]
struct Shape;

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GraphMaterial>>,
) {
    let resolution = 50;
    let step = 2.0 / resolution as f32;

    for i in 0..resolution {
        let x = 1.0 * (i as f32 + 0.5) * step - 1.0;
        let y = x * x * x;

        commands.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 * step })),
                material: materials.add(GraphMaterial {}),
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
        let y = (PI * (x + now)).sin();
        transform.translation.y = y;
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
struct GraphMaterial {}

impl Material for GraphMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/graph_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}
