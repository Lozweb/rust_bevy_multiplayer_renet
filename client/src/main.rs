use bevy::app::{App, PluginGroup, Startup};
use bevy::asset::AssetPlugin;
use bevy::prelude::ImagePlugin;
use bevy::utils::default;
use bevy::window::{WindowPlugin, WindowResolution};
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_renet::RenetClientPlugin;
use client::plugin::client_plugin::ClientPlugin;
use client::plugin::game_plugin::GamePlugin;
use client::system::camera::spawn_camera;

fn main() {
    let mut app = App::new();

    let process_id = std::process::id();
    let window_title = format!("Client - PID: {}", process_id);

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(bevy::window::Window {
                    title: window_title,
                    resolution: WindowResolution::new(1200, 720),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                file_path: "../assets".into(),
                ..default()
            }),
    );

    app.add_plugins(EguiPlugin::default());
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins(RenetClientPlugin);
    app.add_plugins(ClientPlugin);
    app.add_plugins(GamePlugin);

    app.add_systems(Startup, spawn_camera);

    app.run();
}
