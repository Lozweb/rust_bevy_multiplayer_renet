use crate::resource::{ClientLobby, CurrentClientId, PlayerMapping};
use bevy::asset::Assets;
use bevy::log::{error, info};
use bevy::math::Vec3;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::{
    Circle, ColorMaterial, Commands, Entity, MeshMaterial2d, Name, Res, ResMut, Transform,
};
use bevy_renet::renet::{ClientId, RenetClient};
use game_core::client::PlayerEntities;
use game_core::network::deserialize_server_message;
use game_core::player::ControlledPlayer;
use game_core::server::{ServerChannel, ServerMessages};

pub fn client_event_system(
    client_id: Res<CurrentClientId>,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<ClientLobby>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_mapping: ResMut<PlayerMapping>,
) {
    let current_client_id = client_id.0;

    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let (server_message, _size) = deserialize_server_message(&message);
        match server_message {
            ServerMessages::PlayerCreate {
                id,
                translation,
                entity,
            } => {
                let player = player_create_handler(
                    id,
                    Vec3::from(translation),
                    entity,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                );
                if current_client_id == id {
                    commands.entity(player).insert(ControlledPlayer);
                }

                lobby.add_player(
                    &id,
                    PlayerEntities {
                        server_entity: Entity::from_bits(entity),
                        client_entity: player,
                    },
                );
                player_mapping.add(Entity::from_bits(entity), player);
            }
            ServerMessages::PlayerRemove { id } => {
                player_remove_handler(id);
            }
            ServerMessages::Error { message } => {
                error!("Server error message: {}", message);
            }
        }
    }
}

fn player_create_handler(
    id: ClientId,
    position: Vec3,
    entity: u64,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    info!("Player created: {id} at {position:?} with entity {entity}");
    commands
        .spawn((
            Name::new(format!("Player_{id}")),
            Transform::from_translation(position),
            Mesh2d(meshes.add(Mesh::from(Circle::new(40.0)))),
            MeshMaterial2d(materials.add(ColorMaterial::default())),
        ))
        .id()
}

fn player_remove_handler(id: u64) {
    info!("Player removed: {id}");
}
