mod resource;
mod system;

use bevy::app::{App, PluginGroup, Startup};
use bevy::asset::AssetPlugin;
use bevy::prelude::ImagePlugin;
use bevy::utils::default;
use bevy::window::WindowPlugin;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_renet::RenetServerPlugin;
use server::plugin::game_plugin::GamePlugin;
use server::plugin::server_plugin::ServerPlugin;
use server::system::camera::spawn_camera;

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
    );

    app.add_plugins(EguiPlugin::default());
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins(RenetServerPlugin);
    app.add_plugins(ServerPlugin);
    app.add_plugins(GamePlugin);

    app.add_systems(Startup, spawn_camera);

    app.run();
}
