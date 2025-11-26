use bevy::prelude::{Commands, ResMut, Vec3};
use bevy_renet::renet::RenetServer;
use game_core::enemy::{spawn_enemy, Enemy};
use game_core::server::ServerMessages;
use tracing::info;

pub fn spawn_initial_enemies(mut commands: Commands, mut server: ResMut<RenetServer>) {
    info!("spawning initial enemies");
    let position = Vec3::new(200., 0., 0.);
    let e1 = spawn_enemy(&mut commands, position, &mut None, &mut None);

    commands.entity(e1).insert(Enemy { server_entity: e1 });

    info!("initial enemies spawned");

    let message = ServerMessages::EnemySpawned {
        server_entity: e1,
        position,
    };
    ServerMessages::broadcast(&message, &mut server);
    info!("enemy spawn message broadcasted");
}
