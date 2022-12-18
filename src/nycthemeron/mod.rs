mod sun;
pub mod time_of_day;

use bevy::prelude::*;

use self::{
    sun::{SunPlugin, UpdateSunPositionEvent},
    time_of_day::{debug::DebugTimeOfDayPlugin, TimeOfDay, TimeOfDayPlugin},
};

pub struct NycthemeronPlugin;
impl Plugin for NycthemeronPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TimeOfDayPlugin::new(TimeOfDay::new(12., 0., 0., 7200.)))
            .add_plugin(DebugTimeOfDayPlugin)
            .add_plugin(SunPlugin)
            .add_system(time_dependent_sun_position);
    }
}

fn time_dependent_sun_position(
    mut events: EventWriter<UpdateSunPositionEvent>,
    time: Res<TimeOfDay>,
) {
    events.send(UpdateSunPositionEvent(time.fraction_of_day()));
}
