use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::window::PresentMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rustlikecoding::*;

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

const RESOLUTION: i32 = 50;

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GraphMaterial>>,
) {
    let step = 2.0 / RESOLUTION as f32;
    for _ in 0..(RESOLUTION * RESOLUTION) {
        commands.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 * step })),
                material: materials.add(GraphMaterial {}),
                ..default()
            },
            Shape,
        ));
    }

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn update_graph_system(time: Res<Time>, mut query: Query<&mut Transform, With<Shape>>) {
    let t = time.elapsed_seconds();
    let step = 2.0 / RESOLUTION as f32;
    let mut x = 0.0;
    let mut z = 0.0;
    let mut v = 0.5 * step - 1.0;

    for mut transform in &mut query {
        if x == RESOLUTION as f32 {
            x = 0.0;
            z += 1.0;
            v = (z + 0.5) * step - 1.0;
        }

        let u = (x + 0.5) * step - 1.0;

        transform.translation = torus(u, v, t);

        x += 1.0;
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
