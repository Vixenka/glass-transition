use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    str::FromStr,
    time::SystemTime,
};

use bevy::prelude::*;
use bevy_replicon::{
    prelude::*,
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        ConnectionConfig,
    },
};

#[derive(Resource)]
pub struct Client {
    pub id: u64,
}

pub fn start_connection(
    mut commands: Commands,
    address: &str,
    network_channels: Res<NetworkChannels>,
) {
    let client = RenetClient::new(ConnectionConfig {
        server_channels_config: network_channels.get_server_configs(),
        client_channels_config: network_channels.get_client_configs(),
        ..Default::default()
    });

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;

    let ip = IpAddr::V4(
        Ipv4Addr::from_str(address.split(':').next().expect("Missing IP address"))
            .expect("Unable to parse IP address"),
    );
    let server_address = SocketAddr::new(
        ip,
        u16::from_str(address.split(':').last().unwrap_or("13001"))
            .expect("Port is not a u16 number"),
    );
    let socket = UdpSocket::bind((ip, 0)).expect("Unable to bind socket");
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: super::PROTOCOL_ID,
        server_addr: server_address,
        user_data: None,
    };
    let transport = NetcodeClientTransport::new(current_time, authentication, socket)
        .expect("Unable to create client transport");

    info!("Client started on {}", server_address);

    commands.insert_resource(Client { id: client_id });
    commands.insert_resource(client);
    commands.insert_resource(transport);
}
