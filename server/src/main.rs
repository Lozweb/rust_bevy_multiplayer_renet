mod resource;
mod server_plugin;
mod system;

use crate::resource::ServerLobby;
use crate::server_plugin::ServerPlugin;
use bevy::app::{App, PluginGroup};
use bevy::asset::AssetPlugin;
use bevy::prelude::ImagePlugin;
use bevy::utils::default;
use bevy::window::{Window, WindowPlugin};
use bevy::DefaultPlugins;
use bevy_renet::RenetServerPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window { ..default() }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                file_path: "../assets".into(),
                ..default()
            }),
    );

    app.insert_resource(ServerLobby::default());

    app.add_plugins(RenetServerPlugin);
    app.add_plugins(ServerPlugin);

    app.run();
}
