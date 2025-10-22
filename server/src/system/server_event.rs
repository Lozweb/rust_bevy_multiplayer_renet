use crate::resource::ServerLobby;
use bevy::asset::Assets;
use bevy::log::info;
use bevy::math::Vec3;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::{
    Circle, ColorMaterial, Commands, Entity, MeshMaterial2d, MessageReader, Name, Query, ResMut,
    Transform,
};
use bevy_renet::renet::{ClientId, RenetServer, ServerEvent};
use game_core::network::serialize_server_message;
use game_core::player::PlayerInfo;
use game_core::server::{ServerChannel, ServerMessages};

pub fn server_event_system(
    mut player_query: Query<(Entity, &PlayerInfo, &Transform)>,
    mut server_event: MessageReader<ServerEvent>,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in server_event.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                let position = Vec3::new(fastrand::f32() * 800.0 - 400.0, 0.0, 0.0);

                let player_entity = client_connected_handler(
                    client_id,
                    position,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                );

                lobby.add_player(client_id, player_entity);

                send_existing_players_to_client(client_id, &mut player_query, &mut server);
                broadcast_player_create(player_entity, client_id, position, &mut server);
            }
            ServerEvent::ClientDisconnected { client_id, .. } => {
                info!("Client {client_id} disconnected");
                // Additional logic for handling client disconnection can be added here
            }
        }
    }
}

fn client_connected_handler(
    client_id: &u64,
    position: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    info!("Client {client_id} connected");

    commands
        .spawn((
            Name::new(format!("Player_{client_id}")),
            Transform::from_translation(position),
            Mesh2d(meshes.add(Mesh::from(Circle::new(40.0)))),
            MeshMaterial2d(materials.add(ColorMaterial::default())),
        ))
        .id()
}

fn send_existing_players_to_client(
    client_id: &u64,
    player_query: &mut Query<(Entity, &PlayerInfo, &Transform)>,
    server: &mut ResMut<RenetServer>,
) {
    for (entity, player_info, transform) in player_query.iter() {
        let translation: [f32; 3] = transform.translation.into();

        let message = serialize_server_message(&ServerMessages::PlayerCreate {
            id: player_info.id,
            translation,
            entity: entity.to_bits(),
        });
        server.send_message(*client_id, ServerChannel::ServerMessages, message);
    }
}

fn broadcast_player_create(
    player_entity: Entity,
    client_id: &ClientId,
    position: Vec3,
    server: &mut ResMut<RenetServer>,
) {
    let message = serialize_server_message(&ServerMessages::PlayerCreate {
        id: *client_id,
        entity: player_entity.to_bits(),
        translation: position.into(),
    });
    server.broadcast_message(ServerChannel::ServerMessages, message);
}
