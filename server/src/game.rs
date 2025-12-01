use crate::level;
use crate::resource::ServerConfig;
use bevy::app::{App, Startup};
use bevy::prelude::Res;
use tracing::info;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_server);
    app.add_plugins(level::plugin);
}

fn setup_server(config: Res<ServerConfig>) {
    info!(
        "Serveur démarré en mode console sur le port {}",
        config.port
    );
}
