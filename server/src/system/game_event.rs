use crate::resource::ServerLobby;
use bevy::asset::Assets;
use bevy::log::info;
use bevy::mesh::Mesh;
use bevy::prelude::{ColorMaterial, Commands, MessageReader, MessageWriter, ResMut, Vec3};
use bevy_renet::renet::{ClientId, ServerEvent};
use game_core::event::game_event::GameEvent;
use game_core::player::spawn_player;

pub fn on_game_event(
    mut server_event_reader: MessageReader<ServerEvent>,
    mut game_event_writer: MessageWriter<GameEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut lobby: ResMut<ServerLobby>,
) {
    for event in server_event_reader.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                let position = Vec3::new(fastrand::f32() * 800.0 - 400.0, 0.0, 0.0);

                let entity = spawn_player(
                    client_id,
                    position,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                );

                game_event_writer.write(GameEvent::PlayerCreated {
                    client_id: *client_id,
                    entity,
                    position,
                });
            }
            ServerEvent::ClientDisconnected { client_id, .. } => {
                despawn_player(client_id, &mut commands, &mut lobby);

                game_event_writer.write(GameEvent::PlayerRemoved {
                    client_id: *client_id,
                });
            }
        }
    }
}

fn despawn_player(client_id: &ClientId, commands: &mut Commands, lobby: &mut ResMut<ServerLobby>) {
    info!("Client {client_id} disconnected");
    if let Some(entity) = lobby.get_player(&client_id) {
        commands.entity(*entity).despawn();
    }
}
