mod audio;
mod client;
mod dev_tools;
mod game;
mod menu;
mod resource;
mod screens;
mod theme;

use avian2d::prelude::Gravity;
use avian2d::PhysicsPlugins;
use bevy::app::{App, AppExit};
use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::utils::default;
use bevy::window::{WindowPlugin, WindowResolution};
use bevy::DefaultPlugins;
use bevy_renet::RenetClientPlugin;
use game_core::asset_tracking;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        let process_id = std::process::id();
        let window_title = format!("Client - PID: {}", process_id);

        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
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

        app.add_plugins(RenetClientPlugin);

        app.add_plugins((
            asset_tracking::plugin,
            audio::plugin,
            game::plugin,
            PhysicsPlugins::default(),
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            menu::plugin,
            screens::plugin,
            theme::plugin,
        ))
        .insert_resource(Gravity::ZERO);

        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    TickTimers,
    RecordInput,
    Update,
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
struct Pause(pub bool);

#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;
