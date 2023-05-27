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
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::defaultMathFunction())
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(MaterialPlugin::<GraphMaterial>::default())
        .insert_resource(Constants {
            resolution: 50,
            function_duration: 1.0,
            transition_duration: 1.0,
            functions: vec![wave, multi_wave, ripple, sphere, torus],
        })
        .insert_resource(Functions {
            index: 0,
            function: wave,
            transition_function: wave,
        })
        .insert_resource(Duration(1.0))
        .insert_resource(Transitioning(true))
        .add_system(update_graph_system)
        .run();
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GraphMaterial>>,
    constants: Res<Constants>,
) {
    let step = 2.0 / constants.resolution as f32;
    for _ in 0..(constants.resolution * constants.resolution) {
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

fn update_graph_system(
    time: Res<Time>,
    query: Query<&mut Transform, With<Shape>>,
    mut duration: ResMut<Duration>,
    mut transitioning: ResMut<Transitioning>,
    mut functions: ResMut<Functions>,
    constants: Res<Constants>,
) {
    duration.0 += time.raw_delta_seconds();

    if transitioning.0 {
        if duration.0 >= constants.transition_duration {
            duration.0 -= constants.transition_duration;
            transitioning.0 = false;
        }
    } else if duration.0 >= constants.function_duration {
        duration.0 -= constants.function_duration;
        transitioning.0 = true;
        functions.transition_function = functions.function;
        functions.index = (functions.index + 1) % constants.functions.len();
        functions.function = constants.functions[functions.index];
    }

    if transitioning.0 {
        update_function_transitioning(
            time,
            query,
            functions.transition_function,
            functions.function,
            duration.0,
            constants.transition_duration,
            constants.resolution as f32,
        );
    } else {
        update_function(time, query, functions.function, constants.resolution as f32);
    }
}

fn update_function(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Shape>>,
    f: MathFunction,
    resolution: f32,
) {
    let t = time.elapsed_seconds();
    let step = 2.0 / resolution;
    let mut x = 0.0;
    let mut z = 0.0;
    let mut v = 0.5 * step - 1.0;

    for mut transform in &mut query {
        if x == resolution {
            x = 0.0;
            z += 1.0;
            v = (z + 0.5) * step - 1.0;
        }

        let u = (x + 0.5) * step - 1.0;

        transform.translation = f(u, v, t);

        x += 1.0;
    }
}

fn update_function_transitioning(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Shape>>,
    from_f: MathFunction,
    to_f: MathFunction,
    duration: f32,
    transition_duration: f32,
    resolution: f32,
) {
    let progress = duration / transition_duration;

    let t = time.elapsed_seconds();
    let step = 2.0 / resolution;
    let mut x = 0.0;
    let mut z = 0.0;
    let mut v = 0.5 * step - 1.0;

    for mut transform in &mut query {
        if x == resolution {
            x = 0.0;
            z += 1.0;
            v = (z + 0.5) * step - 1.0;
        }

        let u = (x + 0.5) * step - 1.0;

        transform.translation = morph(u, v, t, from_f, to_f, progress);

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

#[derive(Component)]
struct Shape;

#[derive(Resource)]
struct Duration(f32);

#[derive(Resource)]
struct Transitioning(bool);

#[derive(Resource)]
struct Functions {
    index: usize,
    function: MathFunction,
    transition_function: MathFunction,
}

#[derive(Resource)]
struct Constants {
    resolution: i32,
    function_duration: f32,
    transition_duration: f32,
    functions: Vec<MathFunction>,
}
