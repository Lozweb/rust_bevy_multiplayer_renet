mod plugin;
mod system;

// Réexporter le module resource de la lib pour l'utiliser dans les sous-modules
use server::resource;

#[cfg(not(feature = "dev"))]
mod ui {}

#[cfg(feature = "dev")]
mod ui;

use crate::plugin::debug_plugin::DebugPlugin;
use crate::plugin::server_plugin::ServerPlugin;
use bevy::app::{App, PluginGroup};
use bevy::asset::AssetPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::ImagePlugin;
use bevy::utils::default;
use bevy::window::WindowPlugin;
use bevy::{DefaultPlugins, MinimalPlugins};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_renet::RenetServerPlugin;
use clap::Parser;
use server::config::ServerArgs;
use server::resource::ServerConfig;

fn main() {
    let args = ServerArgs::parse();
    let config = ServerConfig::new(args.headless, args.port);
    let headless = config.headless;
    let mut app = App::new();

    if headless {
        app.add_plugins(MinimalPlugins)
            .add_plugins(LogPlugin::default())
            .add_plugins(AssetPlugin {
                file_path: "../assets".into(),
                ..default()
            });
    } else {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin::default())
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    file_path: "../assets".into(),
                    ..default()
                }),
        );

        #[cfg(feature = "dev")]
        {
            app.add_plugins((
                EguiPlugin::default(),
                WorldInspectorPlugin::new(),
                DebugPlugin,
            ));
        }
    }

    // IMPORTANT : Insérer la ressource APRÈS les plugins de base
    app.insert_resource(config);

    app.add_plugins((RenetServerPlugin, ServerPlugin)).run();
}
