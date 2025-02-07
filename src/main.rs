//TODO: Show rectangle

//Rectangle move XY
////Keyboard input
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
const SCREEN_X: f32 = 1080.0;
const SCREEN_Y: f32 = 720.0;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "I am a window!".into(),
            name: Some("bevy.app".into()),
            resolution: (SCREEN_X, SCREEN_Y).into(),
            present_mode: PresentMode::AutoVsync,
            // Tells Wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Dark),
            enabled_buttons: bevy::window::EnabledButtons {
                maximize: true,
                ..Default::default()
            },
            // This will spawn an invisible window
            // The window will be made visible in the make_visible() system after 3 frames.
            // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
            visible: true,
            ..default()
        }),
        ..default()
    }),));
    app.add_systems(Startup, setup);
    app.add_systems(Update, sprite_movement);
    app.run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let shape = meshes.add(Circle::new(51.0));

    let color1 = Color::hsl(180.0, 40.0, 0.5);

    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(color1)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Direction::Right,
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut direction, mut transform) in &mut sprite_position {
        if keys.pressed(KeyCode::KeyA) {
            *direction = Direction::Left;
            transform.translation.x -= 600. * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyD) {
            *direction = Direction::Right;
            transform.translation.x += 600. * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyW) {
            *direction = Direction::Up;
            transform.translation.y += 600. * time.delta_secs();
        }
        if keys.pressed(KeyCode::KeyS) {
            *direction = Direction::Down;
            transform.translation.y -= 600. * time.delta_secs();
        }
    }
} //
