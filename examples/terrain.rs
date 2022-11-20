use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use rustpg::{
    core::{camera::CameraPlugin, spectator::SpectatorPlugin},
    nycthemeron::NycthemeronPlugin,
    terrain::TerrainPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(TerrainPlugin::default())
        .add_plugin(NycthemeronPlugin)
        .add_plugin(SpectatorPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .run();
}
