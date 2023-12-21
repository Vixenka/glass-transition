use std::ops::Mul;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_replicon::{
    network_event::{
        client_event::{ClientEventAppExt, FromClient},
        EventType,
    },
    server::ServerSet,
};
use serde::{Deserialize, Serialize};

use crate::{
    character::{
        enemy::{self, Enemy, EnemyKind},
        player::LocalPlayer,
    },
    network::{
        has_client_and_local_player, has_server, has_server_and_local_player,
        replication::transform::SyncedTransform,
    },
};

pub struct CheatMenuPlugin;

impl Plugin for CheatMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_client_event::<CommandEvent>(EventType::Unordered)
            .add_systems(
                Update,
                (
                    ui_host.run_if(has_server_and_local_player),
                    ui_client.run_if(has_client_and_local_player),
                ),
            )
            .add_systems(
                PreUpdate,
                command_server_handler
                    .after(ServerSet::Receive)
                    .run_if(has_server),
            );
    }
}

fn ui_host(ctx: EguiContexts, commands: Commands, query: Query<&Transform, With<LocalPlayer>>) {
    ui_impl(ctx, commands, None, query);
}

fn ui_client(
    ctx: EguiContexts,
    commands: Commands,
    event: EventWriter<CommandEvent>,
    query: Query<&Transform, With<LocalPlayer>>,
) {
    ui_impl(ctx, commands, Some(event), query);
}

fn ui_impl(
    mut ctx: EguiContexts,
    mut commands: Commands,
    mut event: Option<EventWriter<CommandEvent>>,
    query: Query<&Transform, With<LocalPlayer>>,
) {
    egui::Window::new("Cheat menu").show(ctx.ctx_mut(), |ui| {
        ui.collapsing("Spawn enemies", |ui| {
            for kind in enum_iterator::all::<EnemyKind>() {
                if ui.button(format!("{:?}", kind)).clicked() {
                    push_command(
                        &mut commands,
                        &mut event,
                        CommandEvent::Enemy((Enemy { kind }, near_point(&query).into())),
                    );
                }
            }
        });
    });
}

fn near_point(query: &Query<&Transform, With<LocalPlayer>>) -> Transform {
    let player_transform = query.single();
    Transform::from_translation(player_transform.translation + player_transform.forward().mul(3.0))
}

#[derive(Deserialize, Event, Serialize)]
enum CommandEvent {
    Enemy((Enemy, SyncedTransform)),
}

fn push_command(
    commands: &mut Commands,
    event: &mut Option<EventWriter<CommandEvent>>,
    command: CommandEvent,
) {
    if let Some(event) = event.as_mut() {
        event.send(command);
    } else {
        process_command_event(commands, &command);
    }
}

fn command_server_handler(
    mut commands: Commands,
    mut event: EventReader<FromClient<CommandEvent>>,
) {
    for FromClient {
        client_id: _,
        event,
    } in event.read()
    {
        process_command_event(&mut commands, event)
    }
}

fn process_command_event(commands: &mut Commands, event: &CommandEvent) {
    match event {
        CommandEvent::Enemy((enemy, transform)) => {
            enemy::spawn(commands, enemy.clone(), Transform::from(transform.clone()));
        }
    }
}
