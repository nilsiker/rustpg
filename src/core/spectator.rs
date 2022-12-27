use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Resource)]
struct SpectatorSpeed(f32);

pub struct SpectatorPlugin;
impl Plugin for SpectatorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpectatorSpeed(200.0))
            .add_system(mover)
            .add_system(rotator);
    }
}

fn rotator(
    mut query: Query<&mut Transform, With<Camera>>,
    input: Res<Input<MouseButton>>,
    mut mouse_events: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    if let Ok(mut camera_transform) = query.get_single_mut() {
        for event in mouse_events.iter() {
            if input.pressed(MouseButton::Right) {
                let Vec2 { x, y } = event.delta;

                camera_transform.rotate_y(-x * time.delta_seconds());
                camera_transform.rotate_local_x(-y * time.delta_seconds());
            }
        }
    }
}

fn mover(
    mut query: Query<&mut Transform, With<Camera>>,
    input: Res<Input<KeyCode>>,
    speed: Res<SpectatorSpeed>,
    time: Res<Time>,
) {
    if let Ok(mut camera_transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let mut fast_mode = false;
        for keycode in input.get_pressed() {
            let part_direction = match keycode {
                KeyCode::W => camera_transform.forward(),
                KeyCode::A => camera_transform.left(),
                KeyCode::S => camera_transform.back(),
                KeyCode::D => camera_transform.right(),
                KeyCode::LShift => {
                    fast_mode = true;
                    Vec3::ZERO
                }
                _ => Vec3::ZERO,
            };
            direction += part_direction;
        }

        camera_transform.translation += direction.normalize_or_zero()
            * speed.0
            * time.delta_seconds()
            * if fast_mode { 3.0 } else { 1.0 };
    }
}