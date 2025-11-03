mod plugin;
pub mod resource;
mod system;
mod ui;

use crate::plugin::debug_plugin::DebugPlugin;
use crate::plugin::server_plugin::ServerPlugin;
use bevy::app::{App, PluginGroup};
use bevy::asset::AssetPlugin;
use bevy::prelude::ImagePlugin;
use bevy::utils::default;
use bevy::window::WindowPlugin;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_renet::RenetServerPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin::default())
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                file_path: "../assets".into(),
                ..default()
            }),
    )
    .add_plugins((
        EguiPlugin::default(),
        WorldInspectorPlugin::new(),
        RenetServerPlugin,
        ServerPlugin,
        DebugPlugin,
    ))
    .run();
}
