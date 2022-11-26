use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use rustpg::{
    core::{camera::CameraPlugin, spectator::SpectatorPlugin},
    nycthemeron::NycthemeronPlugin,
    terragen::TerragenPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(115, 186, 215)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(CameraPlugin)
        .add_plugin(NycthemeronPlugin)
        .add_plugin(SpectatorPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .insert_resource(WorldInspectorParams {
            sort_components: true,
            despawnable_entities: true,

            ..default()
        })
        .add_plugin(TerragenPlugin)
        .run();
}
