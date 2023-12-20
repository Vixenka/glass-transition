pub mod client;
pub mod replication;
pub mod server;

use std::str::FromStr;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};
use bevy_replicon::{
    renet::{RenetClient, RenetServer},
    replicon_core::NetworkChannels,
    ReplicationPlugins,
};

use self::server::ServerPlugin;

pub const PROTOCOL_ID: u64 = 0;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ReplicationPlugins)
            .insert_resource(NetworkUiState {
                address: String::from_str("127.0.0.1:13001").unwrap(),
            })
            .add_plugins((replication::ReplicationPlugin, ServerPlugin))
            .add_systems(Update, ui);
    }
}

#[derive(Resource)]
pub struct NetworkUiState {
    address: String,
}

fn ui(
    mut ctx: EguiContexts,
    commands: Commands,
    state: ResMut<NetworkUiState>,
    network_channels: Res<NetworkChannels>,
    server: Option<Res<RenetServer>>,
    client: Option<Res<RenetClient>>,
) {
    egui::Window::new("Network managment").show(ctx.ctx_mut(), |ui| {
        if server.is_none() && client.is_none() {
            ui_connect(state, commands, network_channels, ui);
        }
    });
}

fn ui_connect(
    mut state: ResMut<NetworkUiState>,
    commands: Commands,
    network_channels: Res<NetworkChannels>,
    ui: &mut egui::Ui,
) {
    ui.label("Address");
    ui.text_edit_singleline(&mut state.address);

    if ui.button("Connect").clicked() {
        client::start_connection(commands, &state.address, network_channels);
    } else if ui.button("Host game").clicked() {
        server::start_listening(commands, &state.address, network_channels);
    }
}
