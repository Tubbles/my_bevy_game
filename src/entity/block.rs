use bevy::prelude::*;

#[derive(Debug, Default)]
pub enum BlockType {
    #[default]
    Air,
    Ground,
    //
}

#[derive(Debug, Default)]
pub struct Block {
    pub btype: BlockType,
    pub pos: Vec3,
    pub rot: Vec3,
}

#[derive(Debug, Default)]
struct Textures(Vec<Image>);

struct BlockTextures {
    top: Handle<Image>,
    side: Handle<Image>,
    bottom: Handle<Image>,
}

#[derive(Default)]
pub struct Statics {
    textures: Vec<BlockTextures>,
    meshes: Vec<Handle<Mesh>>,
}

// static mut statics: Statics = Statics {
//     textures: vec![],
//     meshes: vec![],
// };

// create a new quad mesh. this is what we will apply the texture to

impl Block {
    pub fn init(app: &mut App) {
        app.add_startup_system(Self::setup);
        app.init_resource::<Statics>();
    }

    pub fn setup(
        asset_server: Res<AssetServer>,
        mut statics: ResMut<Statics>,
        // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let textures = BlockTextures {
            top: asset_server.load("texture_atlas/ground_top.png"),
            side: asset_server.load("texture_atlas/ground_side.png"),
            bottom: asset_server.load("texture_atlas/ground_bottom.png"),
        };

        statics.textures.push(textures);

        // this material renders the texture normally
        // let material_handle = materials.add(StandardMaterial {
        //     base_color_texture: Some(texture_handle.clone()),
        //     alpha_mode: AlphaMode::Blend,
        //     unlit: true,
        //     ..Default::default()
        // });

        let cube = meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0)));
    }

    // pub fn spawn(mut commands: Commands, btype: BlockType, pos: Vec3, rot: Vec3) {
    //     let b = Block {
    //         btype: BlockType::Air,
    //         pos: Vec3::new(0.0, 0.0, 0.0),
    //         rot: Vec3::new(0.0, 0.0, 0.0),
    //     };
    //     commands.spawn_bundle(PbrBundle {
    //         mesh: quad_handle.clone(),
    //         material: material_handle,
    //         transform: Transform {
    //             translation: Vec3::new(0.0, 0.0, 1.5),
    //             rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
    //             scale: Vec3::splat(scale),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     });
    // }
}
