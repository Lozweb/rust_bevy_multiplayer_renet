#[cfg(not(feature = "dev"))]
mod ui {}

mod config;
mod resource;
mod server;
mod system;

#[cfg(feature = "dev")]
mod debug;

#[cfg(feature = "dev")]
mod ui;

use crate::config::ServerArgs;
use crate::resource::ServerConfig;
use bevy::app::{App, PluginGroup};
use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::log::LogPlugin;
use bevy::prelude::Window;
use bevy::utils::default;
use bevy::window::WindowPlugin;
use bevy::{DefaultPlugins, MinimalPlugins};
use bevy_renet::RenetServerPlugin;
use clap::Parser;

#[cfg(feature = "dev")]
use bevy_egui::EguiPlugin;
#[cfg(feature = "dev")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let args = ServerArgs::parse();
    let config = ServerConfig::new(args.headless, args.port);
    let headless = config.headless;
    let mut app = App::new();

    if headless {
        app.add_plugins(MinimalPlugins)
            .add_plugins(LogPlugin::default())
            .add_plugins(bevy::transform::TransformPlugin)
            .add_plugins(AssetPlugin::default())
            .add_plugins(bevy::scene::ScenePlugin);
    } else {
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );

        #[cfg(feature = "dev")]
        {
            app.add_plugins((
                EguiPlugin::default(),
                WorldInspectorPlugin::new(),
                debug::plugin,
            ));
        }
    }

    app.insert_resource(config);

    app.add_plugins((RenetServerPlugin, server::plugin)).run();
}
