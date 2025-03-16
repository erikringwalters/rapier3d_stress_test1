use bevy::{
    asset::RenderAssetUsages,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    window::PresentMode,
};
use bevy_rapier3d::prelude::*;

// use wasm_thread as thread;


// use std::time::Duration;

// Amount of cubes to spawn (^3)
const CUBE_AXIS_AMOUNT: i32 = 10;

// Physics tick rate
const PHYSICS_HZ: f64 = 60.0;

// Environment
const FLOOR_RADIUS: f32 = 100.0;

// Player Controller
// const MOVEMENT_SPEED: f32 = 10.;
// const ROTATE_SPEED: f32 = 0.05;
// const JUMP_SPEED: f32 = 75.0;
// const GROUND_DISTANCE: f32 = 1.01;
// const JUMP_COOLDOWN: f32 = 0.1;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct PlayerController {
    pub velocity: Velocity,
    pub jump_timer: Timer,
    pub is_on_ground: bool,
}

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(PHYSICS_HZ))
        .add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // let capsule_radius = 0.5;
    // let capsule_half_length = 0.5;
    // let capsule_length = capsule_half_length * 2.0;

    let cube_half_size = 0.4;
    // let cube_size = cube_half_size * 2.0;

    let starting_position_offset = 10.0;

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let light_distance = 1000.0;

    // Directional Light
    commands.spawn((
        DirectionalLight {
            illuminance: 2500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(light_distance, light_distance, -light_distance)
            .looking_at(-Vec3::Y, Vec3::Z),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 2.0, -10.0).looking_at(
            Vec3 {
                x: 0.0,
                y: 3.0,
                z: 0.0,
            },
            Dir3::Y,
        ),
    ));

    // Floor
    let floor = commands
        .spawn((
            RigidBody::Fixed,
            Collider::cylinder(0.5, FLOOR_RADIUS),
            Restitution::coefficient(0.1),
            Transform::from_xyz(0.0, 0.0, 0.0)
        ))
        .id();

    let floor_mesh = commands
        .spawn((
            (
                Mesh3d(meshes.add(Cylinder::new(FLOOR_RADIUS, 1.0))),
                MeshMaterial3d(debug_material.clone()),
            ),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .id();

    commands.entity(floor).add_child(floor_mesh);

    // let color_step = 1.0 / CUBE_AXIS_AMOUNT as f32;

    // Cubes
    for i in 0..CUBE_AXIS_AMOUNT {
        for j in 0..CUBE_AXIS_AMOUNT {
            for k in 0..CUBE_AXIS_AMOUNT {
                let ball = commands
                    .spawn((
                        RigidBody::Dynamic,
                        Collider::cuboid(cube_half_size, cube_half_size, cube_half_size),
                        Restitution::coefficient(0.9),
                        Transform::from_xyz(
                            i as f32 + cube_half_size - (CUBE_AXIS_AMOUNT as f32 / 2.0),
                            j as f32 + starting_position_offset,
                            k as f32 + starting_position_offset / 2.0,
                        )
                    ))
                    .id();
                let ball_mesh = commands
                    .spawn((
                        Mesh3d(meshes.add(Cuboid {
                            half_size: Vec3 {
                                x: cube_half_size,
                                y: cube_half_size,
                                z: cube_half_size,
                            },
                        })),
                        MeshMaterial3d(materials.add(
                            Color::srgba_u8(255, 102, 0, 255)
                        //     Srgba {
                        //     red: i as f32 * color_step,
                        //     green: j as f32 * color_step,
                        //     blue: k as f32 * color_step,
                        //     alpha: 1.0,
                        // }
                    ))),
                    )
                    .id();

                commands.entity(ball).add_child(ball_mesh);
            }
        }
    }
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255,
        255, 159, 102, 255,
        236, 255, 102, 255,
        121, 255, 102, 255,
        102, 255, 198, 255, 
        102, 198, 255, 255, 
        121, 102, 255, 255, 
        236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
