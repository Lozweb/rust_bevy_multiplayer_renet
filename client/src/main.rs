mod client_plugin;

use crate::client_plugin::ClientPlugin;
use bevy::app::{App, PluginGroup};
use bevy::asset::AssetPlugin;
use bevy::prelude::ImagePlugin;
use bevy::utils::default;
use bevy::window::WindowPlugin;
use bevy::DefaultPlugins;
use bevy_renet::RenetClientPlugin;
use client::resource::{ClientLobby, PlayerMapping};

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin { ..default() })
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                file_path: "../assets".into(),
                ..default()
            }),
    );

    app.insert_resource(PlayerMapping::default());
    app.insert_resource(ClientLobby::default());

    app.add_plugins(RenetClientPlugin);
    app.add_plugins(ClientPlugin);
    app.run();
}
