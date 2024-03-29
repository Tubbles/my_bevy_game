#![feature(derive_default_enum)]
// #![feature(stmt_expr_attributes)]

use bevy::{
    app::AppExit,
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    input::ElementState,
    input::{keyboard::KeyCode, Input},
    prelude::*,
    render::camera::CameraPlugin,
    window::WindowMode::*,
};

// use bevy_egui::{egui, EguiContext, EguiPlugin};

use bevy_editor_pls::*;

fn spherical_to_cartesian(spherical: &Vec3) -> Vec3 {
    let (r, theta, phi) = (spherical.x, spherical.y, spherical.z);
    let x = r * phi.cos() * theta.sin();
    let z = r * phi.sin() * theta.sin();
    let y = r * theta.cos();
    Vec3::new(x, y, z)
}

mod entity;
use entity::*;

mod piece;
use piece::*;

mod window;

#[rustfmt::skip]
const RESET_FOCUS: [f32; 3] = [
    14 as f32 / 2.0,
    0.0,
    21 as f32 / 2.0,
];

const SPEED: f32 = 5.0;
const SENS: f32 = 500.0;

#[derive(Default)]
struct MyGame {
    button: bool,
    camera: Vec3,
    pos: Vec3,
    orig_camera: Option<Vec3>,
}

#[derive(Component)]
struct Player(Transform);

// fn ui_example(mut egui_context: ResMut<EguiContext>) {
//     egui::Window::new("debug").show(egui_context.ctx_mut(), |ui| {
//         ui.label("fps: 123");
//     });
// }

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 640. * 2.,
            height: 480. * 2.,
            vsync: false,
            resizable: false,
            mode: BorderlessFullscreen,
            ..Default::default()
        })
        .init_resource::<MyGame>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(create_board)
        .add_startup_system(create_pieces)
        .add_system(exit)
        .add_system(print_mouse_events_system)
        .add_system(wasd)
        // .add_plugin(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_plugin(EditorPlugin)
        // .add_system(ui_example)
        .add_system(camera_writer)
        .add_plugin(EntityCountDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default());

    window::init(&mut app);
    Block::init(&mut app);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut game: ResMut<MyGame>,
    asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut a = Block {
        ..Default::default()
    };
    a.btype = BlockType::Air;
    let size = 32.0;
    let scale = 1.0 / size;

    game.button = false;
    game.camera = Vec3::new(6.00, 0.94, 3.51);
    game.pos = Vec3::new(0.0, 0.0, 0.0);
    game.orig_camera = None;

    let texture_handle = asset_server.load("texture_atlas/ground_side.png");

    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });

    // create a new quad mesh. this is what we will apply the texture to
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(size, size))));

    // textured quad - normal
    commands.spawn_bundle(PbrBundle {
        mesh: quad_handle.clone(),
        material: material_handle,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.5),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
            scale: Vec3::splat(scale),
            ..Default::default()
        },
        ..Default::default()
    });

    // commands
    //     .spawn()
    //     .insert(Player(Transform::from_translation(Vec3::new(
    //         0.0, 0.0, 0.0,
    //     ))))
    //     .with_children(|parent| {
    //         parent
    //             // Camera
    //             .spawn_bundle(PerspectiveCameraBundle {
    //                 transform: Transform::from_matrix(Mat4::from_rotation_translation(
    //                     Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
    //                     Vec3::new(-7.0, 20.0, 4.0),
    //                 ))
    //                 .looking_at(Vec3::from(RESET_FOCUS), Vec3::Y),
    //                 ..Default::default()
    //             });
    //     });

    commands
        // Camera
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            ))
            .looking_at(Vec3::from(RESET_FOCUS), Vec3::Y),
            ..Default::default()
        });

    commands
        // Light
        .spawn_bundle(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}

// Exit the app with ctrl+q
fn exit(input: Res<Input<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));
    let white_material = materials.add(Color::rgb(1., 0.9, 0.9).into());
    let black_material = materials.add(Color::rgb(0., 0.1, 0.1).into());

    // Spawn 64 squares
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                // Change material according to position to get alternating pattern
                material: if (i + j + 1) % 2 == 0 {
                    white_material.clone()
                } else {
                    black_material.clone()
                },
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..Default::default()
            });
        }
    }
}

fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load all the meshes
    let king_mesh: Handle<Mesh> = asset_server.load("model/chess_kit/pieces.glb#Mesh0/Primitive0");
    let king_cross_mesh: Handle<Mesh> =
        asset_server.load("model/chess_kit/pieces.glb#Mesh1/Primitive0");
    let pawn_mesh: Handle<Mesh> = asset_server.load("model/chess_kit/pieces.glb#Mesh2/Primitive0");
    let knight_1_mesh: Handle<Mesh> =
        asset_server.load("model/chess_kit/pieces.glb#Mesh3/Primitive0");
    let knight_2_mesh: Handle<Mesh> =
        asset_server.load("model/chess_kit/pieces.glb#Mesh4/Primitive0");
    let rook_mesh: Handle<Mesh> = asset_server.load("model/chess_kit/pieces.glb#Mesh5/Primitive0");
    let bishop_mesh: Handle<Mesh> =
        asset_server.load("model/chess_kit/pieces.glb#Mesh6/Primitive0");
    let queen_mesh: Handle<Mesh> = asset_server.load("model/chess_kit/pieces.glb#Mesh7/Primitive0");

    // Add some materials
    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());

    for piece in BASE_PIECES {
        let material = match piece.color {
            PieceColor::Black => &white_material,
            PieceColor::White => &black_material,
        };
        let meshes = match piece.ptype {
            PieceType::King => {
                vec![&king_mesh, &king_cross_mesh]
            }
            PieceType::Pawn => {
                vec![&pawn_mesh]
            }
            PieceType::Knight => {
                vec![&knight_1_mesh, &knight_2_mesh]
            }
            PieceType::Rook => {
                vec![&rook_mesh]
            }
            PieceType::Bishop => {
                vec![&bishop_mesh]
            }
            PieceType::Queen => {
                vec![&queen_mesh]
            }
        };
        Piece::spawn(&mut commands, piece, meshes.as_slice(), material);
    }
}

/// This system prints out all mouse events as they come in
fn print_mouse_events_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut game: ResMut<MyGame>,
) {
    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Right {
            game.button = match event.state {
                ElementState::Pressed => true,
                ElementState::Released => false,
            }
        }
    }

    for event in mouse_motion_events.iter() {
        if game.button {
            game.camera.y = (game.camera.y - event.delta.y / SENS)
                .clamp(std::f32::EPSILON, std::f32::consts::PI - std::f32::EPSILON);
            game.camera.z =
                (game.camera.z + event.delta.x / SENS).rem_euclid(std::f32::consts::PI * 2.0);
        }
    }

    for event in mouse_wheel_events.iter() {
        game.camera.x = (game.camera.x - event.y).clamp(1.0, 20.0);
    }
}

/// This system prints out all mouse events as they come in
fn wasd(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut game: ResMut<MyGame>,
    camera_transform: Query<(&Transform, &Camera)>,
) {
    let mut intent = Vec3::new(0.0, 0.0, 0.0);
    if input.pressed(KeyCode::W) {
        intent.x += 1.0;
    }
    if input.pressed(KeyCode::A) {
        intent.z += 1.0;
    }
    if input.pressed(KeyCode::S) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::D) {
        intent.z -= 1.0;
    }
    if input.pressed(KeyCode::Space) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::LControl) {
        intent.y -= 1.0;
    }

    for (transform, camera) in camera_transform.iter() {
        if camera.name == Some(CameraPlugin::CAMERA_3D.to_string()) {
            intent = transform.forward() * intent.x
                + transform.up() * intent.y
                + transform.left() * intent.z;
            // info!("{:?}", transform.forward());
        }
    }

    let next_pos = intent * time.delta_seconds() * SPEED;
    game.pos += next_pos;
}

fn camera_writer(game: Res<MyGame>, mut camera_transform: Query<(&mut Transform, &Camera)>) {
    let new_coords = spherical_to_cartesian(&game.camera);

    // look at that new camera's actual focus
    for (mut transform, camera) in camera_transform.iter_mut() {
        if camera.name == Some(CameraPlugin::CAMERA_3D.to_string()) {
            *transform = Transform::from_translation(new_coords).looking_at(Vec3::ZERO, Vec3::Y);
            transform.translation.x += game.pos.x;
            transform.translation.y += game.pos.y;
            transform.translation.z += game.pos.z;
            // info!("{:?}", transform.translation);
        }
    }
}
