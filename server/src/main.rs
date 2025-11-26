mod config;
mod resource;
mod server;
mod system;

use crate::config::ServerArgs;
use crate::resource::ServerConfig;
use bevy::app::App;
use bevy::log::LogPlugin;
use bevy::MinimalPlugins;
use bevy_renet::RenetServerPlugin;
use clap::Parser;

fn main() {
    let args = ServerArgs::parse();
    let config = ServerConfig::new(args.port);

    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin::default())
        .add_plugins(bevy::transform::TransformPlugin)
        .add_plugins(bevy::asset::AssetPlugin::default())
        .add_plugins(bevy::scene::ScenePlugin);

    app.insert_resource(config);

    app.add_plugins((RenetServerPlugin, server::plugin)).run();
}
