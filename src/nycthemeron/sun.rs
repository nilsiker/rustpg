use bevy::prelude::*;

const TAU: f32 = std::f32::consts::TAU;

pub struct UpdateSunPositionEvent(pub f32);

#[derive(Resource)]
struct FractionOfDay(f32);

/// Adds a sun light to the world, positioned according to the current fraction of day.
pub struct SunPlugin;
impl Plugin for SunPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateSunPositionEvent>()
            .insert_resource(FractionOfDay(0.0))
            .add_startup_system(setup)
            .add_system(handle_events)
            .add_system(update_sun_position);
    }
}

#[derive(Component)]
struct Sun;

fn setup(mut commands: Commands) {
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 32000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Sun);
}

fn handle_events(mut fraction: ResMut<FractionOfDay>, mut events: EventReader<UpdateSunPositionEvent>) {
    for event in events.iter() {
        fraction.0 = event.0;
    }
}

fn update_sun_position(
    mut query: Query<&mut Transform, With<Sun>>,
    fraction_of_day: Res<FractionOfDay>,
) {
    if !fraction_of_day.is_changed() {
        return;
    };

    let Ok(mut transform) = query.get_single_mut() else {
        bevy::log::warn!("no single sun found in scene");
        return
    };

    let target_rot = Quat::from_euler(
        EulerRot::XYZ,
        -TAU / 4.0,
        (TAU * fraction_of_day.0) + (TAU / 2.0),
        0.0,
    );
    transform.rotation = target_rot
}
