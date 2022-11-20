use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use rustpg::{
    core::{camera::CameraPlugin, debug_scene::DebugScenePlugin},
    nycthemeron::NycthemeronPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(DebugScenePlugin)
        .add_plugin(NycthemeronPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}
