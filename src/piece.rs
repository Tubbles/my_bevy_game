use bevy::prelude::*;

pub enum PieceType {
    King,
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
}

pub enum PieceColor {
    Black,
    White,
}

pub struct Piece {
    pub ptype: PieceType,
    pub color: PieceColor,
    pub x: f32,
    pub y: f32,
}

#[rustfmt::skip]
pub const BASE_PIECES : &'static [&'static Piece] = &[
    &Piece{ptype: PieceType::Rook,   color: PieceColor::White, x: 0., y: 0.},
    &Piece{ptype: PieceType::Knight, color: PieceColor::White, x: 0., y: 1.},
    &Piece{ptype: PieceType::Bishop, color: PieceColor::White, x: 0., y: 2.},
    &Piece{ptype: PieceType::Queen,  color: PieceColor::White, x: 0., y: 3.},
    &Piece{ptype: PieceType::King,   color: PieceColor::White, x: 0., y: 4.},
    &Piece{ptype: PieceType::Bishop, color: PieceColor::White, x: 0., y: 5.},
    &Piece{ptype: PieceType::Knight, color: PieceColor::White, x: 0., y: 6.},
    &Piece{ptype: PieceType::Rook,   color: PieceColor::White, x: 0., y: 7.},
    &Piece{ptype: PieceType::Pawn,   color: PieceColor::White, x: 1., y: 0.},
    &Piece{ptype: PieceType::Pawn,   color: PieceColor::White, x: 1., y: 1.},
    &Piece{ptype: PieceType::Pawn,   color: PieceColor::White, x: 1., y: 2.},
    &Piece{ptype: PieceType::Pawn,   color: PieceColor::White, x: 1., y: 3.},
    &Piece{ptype: PieceType::Pawn,   color: PieceColor::White, x: 1., y: 4.},
    &Piece{ptype: PieceType::Pawn,   color: PieceColor::White, x: 1., y: 5.},
    &Piece{ptype: PieceType::Pawn,   color: PieceColor::White, x: 1., y: 6.},
    &Piece{ptype: PieceType::Pawn,   color: PieceColor::White, x: 1., y: 7.},
    &Piece{ptype: PieceType::Rook,   color: PieceColor::Black, x: 7., y: 0.},
    &Piece{ptype: PieceType::Knight, color: PieceColor::Black, x: 7., y: 1.},
    &Piece{ptype: PieceType::Bishop, color: PieceColor::Black, x: 7., y: 2.},
    &Piece{ptype: PieceType::Queen,  color: PieceColor::Black, x: 7., y: 3.},
    &Piece{ptype: PieceType::King,   color: PieceColor::Black, x: 7., y: 4.},
    &Piece{ptype: PieceType::Bishop, color: PieceColor::Black, x: 7., y: 5.},
    &Piece{ptype: PieceType::Knight, color: PieceColor::Black, x: 7., y: 6.},
    &Piece{ptype: PieceType::Rook,   color: PieceColor::Black, x: 7., y: 7.},
    &Piece{ptype: PieceType::Pawn,   color: PieceColor::Black, x: 6., y: 0.},
    &Piece{ptype: PieceType::Pawn,   color: PieceColor::Black, x: 6., y: 1.},
    &Piece{ptype: PieceType::Pawn,   color: PieceColor::Black, x: 6., y: 2.},
    &Piece{ptype: PieceType::Pawn,   color: PieceColor::Black, x: 6., y: 3.},
    &Piece{ptype: PieceType::Pawn,   color: PieceColor::Black, x: 6., y: 4.},
    &Piece{ptype: PieceType::Pawn,   color: PieceColor::Black, x: 6., y: 5.},
    &Piece{ptype: PieceType::Pawn,   color: PieceColor::Black, x: 6., y: 6.},
    &Piece{ptype: PieceType::Pawn,   color: PieceColor::Black, x: 6., y: 7.},
];

impl Piece {
    pub fn spawn(
        commands: &mut Commands,
        piece: &Piece,
        meshes: &[&Handle<Mesh>],
        material: &Handle<StandardMaterial>,
    ) {
        match piece.ptype {
            PieceType::King => {
                commands
                    // Spawn parent entity
                    .spawn_bundle(PbrBundle {
                        transform: Transform::from_translation(Vec3::new(piece.x, 0., piece.y)),
                        ..Default::default()
                    })
                    // Add children to the parent
                    .with_children(|parent| {
                        parent.spawn_bundle(PbrBundle {
                            mesh: meshes[0].clone(),
                            material: material.clone(),
                            transform: {
                                let mut transform =
                                    Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                                transform
                            },
                            ..Default::default()
                        });
                        parent.spawn_bundle(PbrBundle {
                            mesh: meshes[1].clone(),
                            material: material.clone(),
                            transform: {
                                let mut transform =
                                    Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                                transform
                            },
                            ..Default::default()
                        });
                    });
            }
            PieceType::Pawn => {
                commands
                    .spawn_bundle(PbrBundle {
                        transform: Transform::from_translation(Vec3::new(piece.x, 0., piece.y)),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(PbrBundle {
                            mesh: meshes[0].clone(),
                            material: material.clone(),
                            transform: {
                                let mut transform =
                                    Transform::from_translation(Vec3::new(-0.2, 0., 2.6));
                                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                                transform
                            },
                            ..Default::default()
                        });
                    });
            }
            PieceType::Knight => {
                commands
                    // Spawn parent entity
                    .spawn_bundle(PbrBundle {
                        transform: Transform::from_translation(Vec3::new(piece.x, 0., piece.y)),
                        ..Default::default()
                    })
                    // Add children to the parent
                    .with_children(|parent| {
                        parent.spawn_bundle(PbrBundle {
                            mesh: meshes[0].clone(),
                            material: material.clone(),
                            transform: {
                                let mut transform =
                                    Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                                transform
                            },
                            ..Default::default()
                        });
                        parent.spawn_bundle(PbrBundle {
                            mesh: meshes[1].clone(),
                            material: material.clone(),
                            transform: {
                                let mut transform =
                                    Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                                transform
                            },
                            ..Default::default()
                        });
                    });
            }
            PieceType::Rook => {
                commands
                    .spawn_bundle(PbrBundle {
                        transform: Transform::from_translation(Vec3::new(piece.x, 0., piece.y)),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(PbrBundle {
                            mesh: meshes[0].clone(),
                            material: material.clone(),
                            transform: {
                                let mut transform =
                                    Transform::from_translation(Vec3::new(-0.1, 0., 1.8));
                                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                                transform
                            },
                            ..Default::default()
                        });
                    });
            }
            PieceType::Bishop => {
                commands
                    .spawn_bundle(PbrBundle {
                        transform: Transform::from_translation(Vec3::new(piece.x, 0., piece.y)),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(PbrBundle {
                            mesh: meshes[0].clone(),
                            material: material.clone(),
                            transform: {
                                let mut transform =
                                    Transform::from_translation(Vec3::new(-0.1, 0., 0.));
                                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                                transform
                            },
                            ..Default::default()
                        });
                    });
            }
            PieceType::Queen => {
                commands
                    .spawn_bundle(PbrBundle {
                        transform: Transform::from_translation(Vec3::new(piece.x, 0., piece.y)),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(PbrBundle {
                            mesh: meshes[0].clone(),
                            material: material.clone(),
                            transform: {
                                let mut transform =
                                    Transform::from_translation(Vec3::new(-0.2, 0., -0.95));
                                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                                transform
                            },
                            ..Default::default()
                        });
                    });
            }
        };
    }
}
