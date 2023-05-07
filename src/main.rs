use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::window::PresentMode;
use bevy_inspector_egui::egui::mutex::Mutex;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use once_cell::sync::Lazy;
use rand::Rng;
use rustlikecoding::*;

const RESOLUTION: i32 = 50;
const FUNCTION_DURATION: f32 = 1.0;
static DURATION: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(1.0));
static TRANSITIONING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(true));
static FUNCTIONS: Lazy<Vec<MathFunction>> =
    Lazy::new(|| vec![wave, multi_wave, ripple, sphere, torus]);
static FUNCTION: Lazy<Mutex<MathFunction>> = Lazy::new(|| Mutex::new(FUNCTIONS[0]));
static TRANSITION_FUNCTION: Lazy<Mutex<MathFunction>> = Lazy::new(|| Mutex::new(FUNCTIONS[0]));

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

fn update_graph_system(time: Res<Time>, query: Query<&mut Transform, With<Shape>>) {
    let mut duration = DURATION.lock();
    *duration += time.delta_seconds();

    let mut transitioning = TRANSITIONING.lock();
    let mut function = FUNCTION.lock();
    let mut transition_function = TRANSITION_FUNCTION.lock();

    if *transitioning {
        if *duration >= FUNCTION_DURATION {
            *duration -= FUNCTION_DURATION;
            *transitioning = false;
        }
    } else if *duration >= FUNCTION_DURATION {
        *duration -= FUNCTION_DURATION;
        *transitioning = true;
        *transition_function = *function;
        *function = get_random_function();
    }

    if *transitioning {
        update_function_transitioning(time, query, *function, *transition_function, *duration);
    } else {
        update_function(time, query, *transition_function);
    }
}

fn update_function(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Shape>>,
    f: MathFunction,
) {
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

        transform.translation = f(u, v, t);

        x += 1.0;
    }
}

fn update_function_transitioning(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Shape>>,
    f: MathFunction,
    next_f: MathFunction,
    duration: f32,
) {
    let t = time.elapsed_seconds();
    let step = 2.0 / RESOLUTION as f32;
    let mut x = 0.0;
    let mut z = 0.0;
    let mut v = 0.5 * step - 1.0;

    let progress = duration / FUNCTION_DURATION;

    for mut transform in &mut query {
        if x == RESOLUTION as f32 {
            x = 0.0;
            z += 1.0;
            v = (z + 0.5) * step - 1.0;
        }

        let u = (x + 0.5) * step - 1.0;

        transform.translation = morph(u, v, t, f, next_f, progress);

        x += 1.0;
    }
}

fn get_random_function() -> MathFunction {
    let mut rng = rand::thread_rng();
    let function_index = rng.gen_range(0..FUNCTIONS.len());

    return FUNCTIONS[function_index];
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
