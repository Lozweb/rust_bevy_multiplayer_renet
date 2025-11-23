use crate::network::{get_current_time, get_socket, PROTOCOL_ID};
use bevy_renet::netcode::{
    ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication,
    ServerConfig,
};

/// Configure et crée le transport réseau côté serveur.
///
/// # Arguments
///
/// * `ip` - Adresse IP d'écoute
/// * `port` - Port d'écoute
///
/// # Returns
///
/// Transport serveur configuré avec 64 clients max et authentification non sécurisée
///
/// # Panics
///
/// Panique si l'adresse IP:port est invalide
pub fn setup_server_transport(ip: &str, port: u64) -> NetcodeServerTransport {
    let public_addr = format!("{}:{}", ip, port)
        .parse()
        .expect("Failed to parse public address");

    NetcodeServerTransport::new(
        ServerConfig {
            current_time: get_current_time(),
            max_clients: 64,
            protocol_id: PROTOCOL_ID,
            public_addresses: vec![public_addr],
            authentication: ServerAuthentication::Unsecure,
        },
        get_socket(public_addr),
    )
    .unwrap()
}

/// Configure et crée le transport réseau côté client.
///
/// # Arguments
///
/// * `ip` - Adresse IP du serveur
/// * `port` - Port du serveur
///
/// # Returns
///
/// Transport client avec ID unique basé sur le timestamp
///
/// # Panics
///
/// Panique si l'adresse IP:port est invalide
pub fn setup_client_transport(ip: &str, port: u64) -> NetcodeClientTransport {
    let server_addr = format!("{}:{}", ip, port)
        .parse()
        .expect("Failed to parse server address");
    let socket_addr = format!("{}:0", ip)
        .parse()
        .expect("Failed to parse socket address");

    let current_time = get_current_time();
    let client_id = current_time.as_millis() as u64;

    NetcodeClientTransport::new(
        current_time,
        ClientAuthentication::Unsecure {
            client_id,
            protocol_id: PROTOCOL_ID,
            server_addr,
            user_data: None,
        },
        get_socket(socket_addr),
    )
    .unwrap()
}
