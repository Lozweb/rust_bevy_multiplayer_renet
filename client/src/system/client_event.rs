use crate::resource::{ClientLobby, CurrentClientId, PlayerEntities};
use bevy::log::error;
use bevy::prelude::{info, Assets, ColorMaterial, Commands, Mesh, Res, ResMut};
use bevy_renet::renet::RenetClient;
use game_core::network::{MessageDeserialize, ServerChannel};
use game_core::player::{spawn_player, ControlledPlayer};
use game_core::server::ServerMessages;

pub fn on_client_event(
    current_client_id: Res<CurrentClientId>,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<ClientLobby>,
    mut commands: Commands,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    mut materials: Option<ResMut<Assets<ColorMaterial>>>,
) {
    while let Some(event) = client.receive_message(ServerChannel::ServerMessages) {
        match ServerMessages::from_bytes(&event) {
            ServerMessages::PlayerCreate {
                client_id,
                entity,
                position,
            } => {
                if lobby.get_player_by_server_entity(&entity).is_none() {
                    info!("Player created: {client_id} at {position:?} with entity {entity}");

                    let player = spawn_player(
                        &client_id,
                        position,
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                    );

                    if current_client_id.0 == client_id {
                        commands.entity(player).insert(ControlledPlayer);
                    }

                    lobby.add_player(
                        &client_id,
                        PlayerEntities {
                            server_entity: entity,
                            client_entity: player,
                        },
                    );
                }
            }
            ServerMessages::PlayerRemove { client_id } => {
                info!("Player removed: {client_id}");
                if let Some(PlayerEntities {
                    server_entity: _server_entity,
                    client_entity,
                }) = lobby.remove_player(&client_id)
                {
                    commands.entity(client_entity).despawn();
                }
            }
            ServerMessages::ErrorMessage { reason } => {
                error!("{}", reason);
            }
        }
    }
}

pub fn server_network_sync(mut client: ResMut<RenetClient>) {
    while let Some(event) = client.receive_message(ServerChannel::NetworkedEntities) {
        ServerMessages::from_bytes(&event);
        { /* Ignore other messages */ }
    }
}
