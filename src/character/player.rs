use bevy::{ecs::system::EntityCommands, math::vec3, prelude::*};
use bevy_replicon::{
    client::ClientSet,
    network_event::{
        client_event::{ClientEventAppExt, FromClient},
        server_event::{SendMode, ServerEventAppExt, ToClients},
        EventType,
    },
    replicon_core::replication_rules::{AppReplicationExt, Ignored},
    server::ServerSet,
};
use serde::{Deserialize, Serialize};

use crate::network::{
    client::{Client, ClientId},
    has_client, has_client_and_local_player, has_local_player, has_server,
    replication::transform::SyncedTransform,
};

use super::{
    appearance::{CharacterAppearanceAssets, CharacterAppearanceBundle, RotateTowardsCamera},
    CharacterPhysicsBundle, CharacterVectors, MoveCharacters,
};

pub const RADIUS: f32 = 0.4;
pub const HALF_HEIGHT: f32 = 0.4;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_server_event::<TransformServerEvent>(EventType::Ordered)
            .add_client_event::<TransformClientEvent>(EventType::Ordered)
            .replicate::<Player>()
            .add_systems(
                PreUpdate,
                (
                    init_players.after(ClientSet::Receive).run_if(has_client),
                    transform_server_handler
                        .after(ServerSet::Receive)
                        .run_if(has_server),
                    transform_client_handler
                        .after(ClientSet::Receive)
                        .run_if(has_client),
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    control.run_if(has_local_player).before(MoveCharacters),
                    transform_server_sender.run_if(has_server),
                    transform_client_sender.run_if(has_client_and_local_player),
                ),
            );
    }
}

#[derive(Copy, Clone)]
pub enum PlayerKind {
    Local,
    Remote,
}

pub fn spawn(
    commands: &mut Commands,
    character_appearances: &CharacterAppearanceAssets,
    player: Player,
    kind: PlayerKind,
) {
    let transform = Transform::from_xyz(0.0, 3.0, 0.0);
    let mut entity_commands = commands.spawn((
        player,
        SharedPlayerBundle::new(character_appearances, transform),
        Ignored::<Transform>::default(),
    ));

    add_kind_dependent_components_to_players(&mut entity_commands, kind, transform);
}

fn init_players(
    mut commands: Commands,
    character_appearances: Res<CharacterAppearanceAssets>,
    spawned_players: Query<(Entity, &Player), Added<Player>>,
    client: Res<Client>,
) {
    for (entity, player) in &spawned_players {
        let kind = match player.client_id == client.id {
            true => PlayerKind::Local,
            false => PlayerKind::Remote,
        };

        let transform = Transform::from_xyz(0.0, 3.0, 0.0);
        let mut c = commands.entity(entity);
        let entity_commands = c.insert(SharedPlayerBundle::new(&character_appearances, transform));

        add_kind_dependent_components_to_players(entity_commands, kind, transform);
    }
}

fn add_kind_dependent_components_to_players(
    entity_commands: &mut EntityCommands,
    kind: PlayerKind,
    transform: Transform,
) {
    match kind {
        PlayerKind::Local => {
            entity_commands.insert(LocalPlayerBundle {
                local_player: LocalPlayer,
                character_physics: CharacterPhysicsBundle::new(HALF_HEIGHT, RADIUS),
            });
            entity_commands
                .commands()
                .insert_resource(LocalPlayerResource);
        }
        PlayerKind::Remote => {
            entity_commands.insert(RemotePlayerBundle {
                synced_transform: transform.into(),
            });
        }
    };
}

#[derive(Component, Serialize, Deserialize)]
pub struct Player {
    pub client_id: ClientId,
}

#[derive(Component, Default)]
pub struct LocalPlayer;

#[derive(Resource)]
pub struct LocalPlayerResource;

#[derive(Bundle)]
struct LocalPlayerBundle {
    local_player: LocalPlayer,
    character_physics: CharacterPhysicsBundle,
}

#[derive(Bundle)]
struct SharedPlayerBundle {
    transform: TransformBundle,
    character_physics: CharacterPhysicsBundle,
    character_appearance: CharacterAppearanceBundle,
}

#[derive(Bundle)]
struct RemotePlayerBundle {
    synced_transform: SyncedTransform,
}

impl SharedPlayerBundle {
    pub fn new(
        character_appearances: &CharacterAppearanceAssets,
        transform: Transform,
    ) -> SharedPlayerBundle {
        Self {
            transform: TransformBundle::from_transform(transform),
            character_physics: CharacterPhysicsBundle::new(RADIUS, HALF_HEIGHT),
            character_appearance: CharacterAppearanceBundle {
                mesh: character_appearances.plane_mesh.clone(),
                material: character_appearances.player_material.clone(),
                visibility: default(),
                rotate_towards_camera: RotateTowardsCamera,
            },
        }
    }
}

fn control(mut query: Query<&mut CharacterVectors, With<LocalPlayer>>, input: Res<Input<KeyCode>>) {
    let mut vectors = query.single_mut();
    vectors.velocity += vec3(0.0, -0.005, 0.0);

    let mut movement = Vec3::ZERO;
    if input.pressed(KeyCode::A) {
        movement += vec3(-1.0, 0.0, 1.0);
    }
    if input.pressed(KeyCode::S) {
        movement += vec3(1.0, 0.0, 1.0);
    }
    if input.pressed(KeyCode::D) {
        movement += vec3(1.0, 0.0, -1.0);
    }
    if input.pressed(KeyCode::W) {
        movement += vec3(-1.0, 0.0, -1.0);
    }
    let speed = 0.015;
    movement = movement.normalize_or_zero() * speed;
    vectors.velocity += movement;

    let damping = 0.8;
    vectors.velocity.x *= damping;
    vectors.velocity.z *= damping;
}

#[derive(Deserialize, Event, Serialize)]
struct TransformServerEvent {
    client_id: ClientId,
    transform: SyncedTransform,
}

#[derive(Deserialize, Event, Serialize)]
struct TransformClientEvent {
    transform: SyncedTransform,
}

fn transform_server_sender(
    mut event: EventWriter<ToClients<TransformServerEvent>>,
    query: Query<(&Transform, &Player)>,
) {
    for (transform, player) in &mut query.iter() {
        event.send(ToClients {
            mode: SendMode::Broadcast,
            event: TransformServerEvent {
                client_id: player.client_id,
                transform: (*transform).into(),
            },
        });
    }
}

fn transform_client_handler(
    mut event: EventReader<TransformServerEvent>,
    mut query: Query<(&Player, &mut SyncedTransform), Without<LocalPlayer>>,
) {
    for event in event.read() {
        // Ignore LocalPlayer.
        if let Some((_, mut transform)) =
            query.iter_mut().find(|x| x.0.client_id == event.client_id)
        {
            *transform = event.transform.clone();
        }
    }
}

fn transform_client_sender(
    mut event: EventWriter<TransformClientEvent>,
    query: Query<&Transform, With<LocalPlayer>>,
) {
    let transform = query.single();
    event.send(TransformClientEvent {
        transform: (*transform).into(),
    });
}

fn transform_server_handler(
    mut event: EventReader<FromClient<TransformClientEvent>>,
    mut query: Query<(&Player, &mut SyncedTransform)>,
) {
    for FromClient { client_id, event } in event.read() {
        let (_, mut transform) = query
            .iter_mut()
            .find(|x| x.0.client_id == client_id)
            .expect("Expecting player to exist");

        *transform = event.transform.clone();
    }
}
