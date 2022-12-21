mod sun;
pub mod time_of_day;

use bevy::prelude::*;
use bevy_inspector_egui::InspectorPlugin;

use self::{
    sun::{SunPlugin, UpdateSunPositionEvent},
    time_of_day::{debug::DebugTimeOfDayPlugin, TimeOfDay, TimeOfDayPlugin},
};

#[derive(Default)]
pub struct NycthemeronPlugin {
    pub time_of_day: TimeOfDay,
    pub inspectors: bool,
}
impl Plugin for NycthemeronPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TimeOfDayPlugin::new(self.time_of_day.clone()))
            .add_plugin(DebugTimeOfDayPlugin)
            .add_plugin(SunPlugin)
            .add_system(time_dependent_sun_position);

        if self.inspectors {
            app.add_plugin(InspectorPlugin::<TimeOfDay>::new_insert_manually());
        }
    }
}

fn time_dependent_sun_position(
    mut events: EventWriter<UpdateSunPositionEvent>,
    time: Res<TimeOfDay>,
) {
    events.send(UpdateSunPositionEvent(time.fraction_of_day()));
}
