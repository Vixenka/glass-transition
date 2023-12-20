use bevy::{math::vec3, prelude::*};
use bevy_replicon::{
    client::ClientSet,
    network_event::{
        client_event::{ClientEventAppExt, FromClient},
        server_event::{SendMode, ServerEventAppExt, ToClients},
        EventType,
    },
    replicon_core::replication_rules::{AppReplicationExt, Ignored},
    server::{ServerSet, SERVER_ID},
};
use serde::{Deserialize, Serialize};

use crate::network::{
    client::Client, has_client, has_client_and_local, has_local_player, has_server,
    replication::transform::SyncedTransform,
};

use super::{CharacterPhysicsBundle, CharacterVectors};

pub const RADIUS: f32 = 0.4;
pub const HALF_HEIGHT: f32 = 0.4;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_server_event::<TransformEvent>(EventType::Ordered)
            .add_client_event::<TransformEvent>(EventType::Ordered)
            .replicate::<Player>()
            .add_systems(
                PreUpdate,
                (
                    init_players.after(ClientSet::Receive).run_if(has_client()),
                    transform_server_handler
                        .after(ServerSet::Receive)
                        .run_if(has_server()),
                    transform_client_handler
                        .after(ClientSet::Receive)
                        .run_if(has_client()),
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    control.run_if(has_local_player()),
                    transform_server_sender.run_if(has_server()),
                    transform_client_sender.run_if(has_client_and_local()),
                ),
            );
    }
}

pub fn spawn(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    player: Player,
    is_local: bool,
) {
    let transform = Transform::from_xyz(0.0, 3.0, 0.0);
    let mut c = commands.spawn((
        player,
        SharedPlayerBundle::new(meshes, materials, transform, is_local),
        Ignored::<Transform>::default(),
        Ignored::<CharacterVectors>::default(),
    ));

    if is_local {
        c.insert(LocalPlayerBundle::default());
        commands.insert_resource(LocalPlayerResource);
    } else {
        c.insert(RemotePlayerBundle {
            synced_transform: transform.into(),
        });
    }
}

fn init_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    spawned_players: Query<(Entity, &Player), Added<Player>>,
    client: Res<Client>,
) {
    for (entity, player) in &spawned_players {
        let transform = Transform::from_xyz(0.0, 3.0, 0.0);
        let mut c = commands.entity(entity);
        c.insert(SharedPlayerBundle::new(
            &mut meshes,
            &mut materials,
            transform,
            player.client_id == client.id,
        ));

        if player.client_id == client.id {
            c.insert(LocalPlayerBundle::default());
            commands.insert_resource(LocalPlayerResource);
        } else {
            c.insert(RemotePlayerBundle {
                synced_transform: transform.into(),
            });
        }
    }
}

#[derive(Component, Serialize, Deserialize)]
pub struct Player {
    pub client_id: u64,
}

#[derive(Component, Default)]
pub struct LocalPlayer;

#[derive(Resource)]
pub struct LocalPlayerResource;

#[derive(Bundle, Default)]
struct LocalPlayerBundle {
    local_player: LocalPlayer,
}

#[derive(Bundle)]
struct SharedPlayerBundle {
    transform: TransformBundle,
    character_physics: CharacterPhysicsBundle,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    visibility: VisibilityBundle,
}

#[derive(Bundle)]
struct RemotePlayerBundle {
    synced_transform: SyncedTransform,
}

impl SharedPlayerBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        transform: Transform,
        is_local: bool,
    ) -> SharedPlayerBundle {
        Self {
            transform: TransformBundle::from_transform(transform),
            character_physics: CharacterPhysicsBundle::new(RADIUS, HALF_HEIGHT),
            mesh: meshes.add(
                shape::Cylinder {
                    radius: RADIUS,
                    height: HALF_HEIGHT * 2.0,
                    resolution: 16,
                    segments: 1,
                }
                .into(),
            ),
            material: materials.add(
                match is_local {
                    true => Color::WHITE,
                    false => Color::GRAY,
                }
                .into(),
            ),
            visibility: VisibilityBundle::default(),
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
struct TransformEvent {
    client_id: u64,
    transform: SyncedTransform,
}

fn transform_server_sender(
    mut event: EventWriter<ToClients<TransformEvent>>,
    query: Query<(&Transform, &Player)>,
) {
    for (transform, player) in &mut query.iter() {
        event.send(ToClients {
            mode: SendMode::BroadcastExcept(SERVER_ID),
            event: TransformEvent {
                client_id: player.client_id,
                transform: (*transform).into(),
            },
        });
    }
}

fn transform_client_handler(
    mut event: EventReader<TransformEvent>,
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
    mut event: EventWriter<TransformEvent>,
    query: Query<(&Transform, &Player), With<LocalPlayer>>,
) {
    let (transform, player) = query.single();
    event.send(TransformEvent {
        client_id: player.client_id,
        transform: (*transform).into(),
    });
}

fn transform_server_handler(
    mut event: EventReader<FromClient<TransformEvent>>,
    mut query: Query<(&Player, &mut SyncedTransform)>,
) {
    for FromClient { client_id, event } in event.read() {
        let (_, mut transform) = query
            .iter_mut()
            .find(|x| x.0.client_id == client_id.raw())
            .expect("Expecting player to exist");

        *transform = event.transform.clone();
    }
}
