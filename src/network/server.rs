use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::*;
use bevy_replicon::{
    prelude::*,
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        ConnectionConfig, ServerEvent,
    },
};

use crate::character::{
    appearance::CharacterAppearanceAssets,
    player::{self, Player},
};

use super::network_error::NetworkError;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            process_server_events.run_if(resource_exists::<RenetServer>()),
        );
    }
}

#[derive(Resource)]
pub struct Server;

pub fn start_listening(
    mut commands: Commands,
    character_appearances: &CharacterAppearanceAssets,
    network_channels: &NetworkChannels,
    server_port: u16,
) -> Result<(), NetworkError> {
    let server = RenetServer::new(ConnectionConfig {
        server_channels_config: network_channels.get_server_configs(),
        client_channels_config: network_channels.get_client_configs(),
        ..default()
    });

    let public_address = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), server_port);
    let socket = UdpSocket::bind(public_address).map_err(|_| NetworkError::UnableBindSocket)?;
    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: 2,
        protocol_id: super::PROTOCOL_ID,
        authentication: ServerAuthentication::Unsecure,
        public_addresses: vec![public_address],
    };
    let transport = NetcodeServerTransport::new(server_config, socket)
        .map_err(|_| NetworkError::UnableCreateServerTransport)?;

    info!("Server started on {}", public_address);

    commands.insert_resource(server);
    commands.insert_resource(transport);
    commands.insert_resource(Server);

    player::spawn(
        &mut commands,
        character_appearances,
        player::Player {
            client_id: SERVER_ID.into(),
        },
        player::PlayerKind::Local,
    );

    Ok(())
}

fn process_server_events(
    mut server_event: EventReader<ServerEvent>,
    mut commands: Commands,
    character_appearances: Res<CharacterAppearanceAssets>,
    query: Query<(Entity, &Player)>,
) {
    for event in server_event.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("Player {client_id} connected.");

                player::spawn(
                    &mut commands,
                    &character_appearances,
                    player::Player {
                        client_id: (*client_id).into(),
                    },
                    player::PlayerKind::Remote,
                );
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Player {client_id} disconnected: {reason}");

                query
                    .iter()
                    .filter(|x| x.1.client_id == client_id)
                    .for_each(|x| {
                        commands.entity(x.0).despawn_recursive();
                    });
            }
        }
    }
}
