use bevy::{prelude::*, window::WindowMode};

pub fn init(app: &mut App) {
    app.init_resource::<window::PrevWindow>()
        .add_system(toggle_fullscreen)
        .add_system(window::update_window);
}

mod window {
    use bevy::{prelude::*, window::WindowMode};

    #[derive(Default)]
    pub struct PrevWindow(WindowDescriptor);

    #[inline]
    fn compare_f32(a: f32, b: f32) -> bool {
        (a - b).abs() < std::f32::EPSILON
    }

    pub fn update_window(
        mut prev_window: ResMut<PrevWindow>,
        curr: Res<WindowDescriptor>,
        mut windows: ResMut<Windows>,
    ) {
        if curr.is_changed() {
            let window = windows.get_primary_mut().unwrap();
            let prev = &prev_window.0;

            if compare_f32(prev.width, curr.width) || compare_f32(prev.height, curr.height) {
                window.set_resolution(curr.width, curr.height);
            }
            if prev.scale_factor_override != curr.scale_factor_override {
                window.set_scale_factor_override(curr.scale_factor_override);
            }
            if prev.title != curr.title {
                window.set_title(curr.title.clone());
            }
            if prev.vsync != curr.vsync {
                window.set_vsync(curr.vsync);
            }
            if prev.resizable != curr.resizable {
                window.set_resizable(curr.resizable);
            }
            if prev.decorations != curr.decorations {
                window.set_decorations(curr.decorations);
            }
            if prev.cursor_visible != curr.cursor_visible {
                window.set_cursor_visibility(curr.cursor_visible);
            }
            if prev.cursor_locked != curr.cursor_locked {
                window.set_cursor_lock_mode(curr.cursor_locked);
            }
            match (prev.mode, curr.mode) {
                (WindowMode::Windowed, WindowMode::Windowed)
                | (WindowMode::BorderlessFullscreen, WindowMode::BorderlessFullscreen) => {}
                _ => {
                    window.set_mode(curr.mode);
                }
            }

            prev_window.0 = curr.clone();
        }
    }
}

fn toggle_fullscreen(input: Res<Input<KeyCode>>, mut window_descriptor: ResMut<WindowDescriptor>) {
    if input.just_pressed(KeyCode::F10) {
        window_descriptor.mode = match window_descriptor.mode {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen,
            WindowMode::BorderlessFullscreen => WindowMode::Windowed,
            _ => unreachable!(),
        }
    }
}
