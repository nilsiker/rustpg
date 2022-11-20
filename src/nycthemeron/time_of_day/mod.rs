pub mod debug;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

const MAX_SECONDS: f32 = 86400.0;
const SECONDS_IN_MINUTE: f32 = 60.0;
const MINUTES_IN_HOUR: f32 = 60.0;
const SECONDS_IN_HOUR: f32 = 3600.0;

#[derive(Debug)]
pub enum PartOfDay {
    Morning,
    Day,
    Evening,
    Night,
}

#[derive(Default)]
pub struct TimeOfDayPlugin {
    time_of_day: TimeOfDay,
}
impl TimeOfDayPlugin {
    pub fn new(initial_time: TimeOfDay) -> Self {
        Self {
            time_of_day: initial_time,
        }
    }
}
impl Plugin for TimeOfDayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.time_of_day.clone())
            .add_system_to_stage(CoreStage::PreUpdate, pass_time);
    }
}

/// A struct to hold the current time of day, assuming a 24-hour day.
///
/// Defaults to starting time 08:00:00, in perfect time for school!
#[derive(Resource, Clone, Inspectable, Reflect, Debug)]
pub struct TimeOfDay {
    time: f32,
    speed_modifier: f32,
}

impl Default for TimeOfDay {
    fn default() -> Self {
        Self {
            time: 8.0 * SECONDS_IN_HOUR,
            speed_modifier: 1.0,
        }
    }
}

impl From<&TimeOfDay> for PartOfDay {
    fn from(time: &TimeOfDay) -> Self {
        let hours = time.hours();

        if hours > 5.0 && hours <= 11.0 {
            Self::Morning
        } else if hours > 11.0 && hours <= 17.0 {
            Self::Day
        } else if hours > 17.0 && hours <= 22.0 {
            Self::Evening
        } else {
            Self::Night
        }
    }
}

impl TimeOfDay {
    /// Creates a time of day, setting the initial time to hh:mm:ss.
    /// # Arguments
    /// * `h` - the starting hour mark.
    /// * `m` - the starting minute mark.
    /// * `s` - the starting second mark.
    /// * `speed` - a multiplier controlling the rate at which time passes.
    pub fn new(h: f32, m: f32, s: f32, speed: f32) -> TimeOfDay {
        TimeOfDay {
            time: h * 3600.0 + m * 60.0 + s,
            speed_modifier: speed,
        }
    }

    /// A convenience getter, returning a tuple containing the current time in the format (h, m, s).
    pub fn time(&self) -> (f32, f32, f32) {
        (self.hours(), self.minutes(), self.seconds())
    }

    pub fn fraction_of_day(&self) -> f32 {
        self.time / MAX_SECONDS
    }

    pub fn seconds(&self) -> f32 {
        (self.time % SECONDS_IN_HOUR % SECONDS_IN_MINUTE).trunc()
    }

    pub fn minutes(&self) -> f32 {
        (self.time / SECONDS_IN_MINUTE % MINUTES_IN_HOUR).trunc()
    }

    pub fn hours(&self) -> f32 {
        (self.time / SECONDS_IN_HOUR).trunc()
    }

    pub fn get_part(&self) -> PartOfDay {
        self.into()
    }

    fn tick(&mut self, delta_seconds: f32) {
        self.time += delta_seconds * self.speed_modifier;
        if self.time > MAX_SECONDS {
            self.time %= MAX_SECONDS
        }
    }
}

fn pass_time(mut time_of_day: ResMut<TimeOfDay>, time: Res<Time>) {
    time_of_day.tick(time.delta_seconds());
}

fn _log_time(time_of_day: ResMut<TimeOfDay>) {
    let (h, m, s) = time_of_day.time();
    bevy::log::info!("Current time of day: {:02.0}:{:02.0}:{:02.0}", h, m, s);
}
