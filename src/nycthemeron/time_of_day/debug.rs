use bevy::prelude::*;

use super::TimeOfDay;

pub struct DebugTimeOfDayPlugin;
impl Plugin for DebugTimeOfDayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_time_ui)
            .add_system(update_time_ui);
    }
}

#[derive(Component)]
struct TimeText;

fn setup_time_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "INIT TIME_OF_DAY",
            TextStyle {
                font: asset_server.load("fonts/CascadiaCode-Regular.otf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        TimeText,
    ));
}

fn update_time_ui(time: Res<TimeOfDay>, mut query: Query<&mut Text, With<TimeText>>) {
    for mut text in &mut query {
        let (h, m, s) = time.time();
        text.sections[0].value = format!("{:?}: {:02.0}:{:02.0}:{:02.0}", time.get_part(), h, m, s);
    }
}
