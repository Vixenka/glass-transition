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
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

use crate::{
    character::{
        enemy::{self, DummyEnemy, Enemy},
        player::LocalPlayer,
    },
    network::{has_local_player, has_server, replication::transform::SyncedTransform},
};

pub struct CheatMenuPlugin;

impl Plugin for CheatMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_client_event::<CommandEvent>(EventType::Unordered)
            .add_systems(Update, (ui.run_if(has_local_player),))
            .add_systems(
                PreUpdate,
                command_server_handler
                    .after(ServerSet::Receive)
                    .run_if(has_server),
            );
    }
}

#[derive(Clone, Copy, Deserialize, Serialize, Sequence, Debug)]
enum EnemyKind {
    Dummy,
}

fn ui(
    mut ctx: EguiContexts,
    mut event: EventWriter<CommandEvent>,
    query: Query<&Transform, With<LocalPlayer>>,
) {
    egui::Window::new("Cheat menu").show(ctx.ctx_mut(), |ui| {
        ui.collapsing("Spawn enemies", |ui| {
            for kind in enum_iterator::all::<EnemyKind>() {
                if ui.button(format!("{:?}", kind)).clicked() {
                    event.send(CommandEvent::Enemy((
                        Enemy,
                        kind,
                        near_point(query.single()).into(),
                    )));
                }
            }
        });
    });
}

fn near_point(player_transform: &Transform) -> Transform {
    Transform::from_translation(player_transform.translation + player_transform.forward().mul(3.0))
}

#[derive(Deserialize, Event, Serialize)]
enum CommandEvent {
    Enemy((Enemy, EnemyKind, SyncedTransform)),
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
        match event {
            CommandEvent::Enemy((enemy, kind, transform)) => {
                enemy::spawn(
                    &mut commands,
                    enemy.clone(),
                    Transform::from(transform.clone()),
                )
                .insert(match kind {
                    EnemyKind::Dummy => DummyEnemy,
                });
            }
        }
    }
}
