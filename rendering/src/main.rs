use bevy::prelude::shape::Quad;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AddressMode, AsBindGroup, SamplerDescriptor, ShaderRef};
use bevy::window::PresentMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_system(spawn_basic_scene)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Rust Like Coding".into(),
                        present_mode: PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: SamplerDescriptor {
                        address_mode_u: AddressMode::Repeat,
                        address_mode_v: AddressMode::Repeat,
                        address_mode_w: AddressMode::Repeat,
                        ..Default::default()
                    },
                }),
        )
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        MaterialMeshBundle {
            // mesh: meshes.add(
            //     Mesh::try_from(shape::Icosphere {
            //         radius: 7.0,
            //         subdivisions: 36,
            //     })
            //     .unwrap(),
            // ),
            // mesh: meshes.add(Mesh::from(shape::Cube { size: 10.0 })),
            mesh: meshes.add(Mesh::from(Quad::new(Vec2 { x: 10.0, y: 10.0 }))),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(CustomMaterial {
                color: Color::SEA_GREEN,
                marble_texture: Some(asset_server.load("marble_tex.png")),
                marble_detail_texture: Some(asset_server.load("marble_detail_tex.png")),
                grid_texture: Some(asset_server.load("distorted_grid_tex.png")),
                grid_detail_texture: Some(asset_server.load("grid_detail_tex.png")),
                splat_map_texture: Some(asset_server.load("binary_splat_map.png")),
            }),
            ..default()
        },
        Shape,
    ));

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
    //     transform: Transform::from_xyz(4.0, 8.0, 7.0),
    //     ..default()
    // });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

#[derive(Component)]
struct Shape;

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    grid_texture: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    grid_detail_texture: Option<Handle<Image>>,
    #[texture(5)]
    #[sampler(6)]
    marble_texture: Option<Handle<Image>>,
    #[texture(7)]
    #[sampler(8)]
    marble_detail_texture: Option<Handle<Image>>,
    #[texture(9)]
    #[sampler(10)]
    splat_map_texture: Option<Handle<Image>>,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}
