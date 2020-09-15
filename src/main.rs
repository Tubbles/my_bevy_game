use bevy::prelude::*;

struct Movable {
    speed: f32,
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(movement.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let gabe_handle = asset_server
        .load_sync(
            &mut textures,
            "assets/gabe-idle-run.png",
        )
        .unwrap();
    let gabe = textures.get(&gabe_handle).unwrap();
    let gabe_atlas = TextureAtlas::from_grid(gabe_handle, gabe.size, 7, 1);
    let gabe_atlas_handle = texture_atlases.add(gabe_atlas);

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: gabe_atlas_handle,
            scale: Scale(3.0),
            translation: Translation::new(-3.0, -10.0, 10.0),
            ..Default::default()
        })
        .with(Movable { speed: 500.0 });
}

fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut q: Query<(&mut Movable, &mut Translation)>,
) {
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    for (m, mut t) in &mut q.iter() {
        *t.0.x_mut() += time.delta_seconds * m.speed * direction;
    }
}
