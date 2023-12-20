use bevy::{math::vec3, prelude::*};
use bevy_replicon::{
    client::ClientSet,
    network_event::{
        client_event::{ClientEventAppExt, FromClient},
        EventType,
    },
    replicon_core::replication_rules::AppReplicationExt,
    server::has_authority,
};
use serde::{Deserialize, Serialize};

use super::{CharacterPhysicsBundle, CharacterVectors};

pub const RADIUS: f32 = 0.4;
pub const HALF_HEIGHT: f32 = 0.4;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_client_event::<ControlEvent>(EventType::Ordered)
            .replicate::<Player>()
            .add_systems(PreUpdate, init_players.after(ClientSet::Receive))
            .add_systems(
                FixedUpdate,
                (control, control_handler.run_if(has_authority())),
            );
    }
}

pub fn spawn(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    player: Player,
) {
    commands.spawn((
        player,
        TransformBundle::from_transform(Transform::from_xyz(0.0, 3.0, 0.0)),
        PlayerSharedBundle::new(meshes, materials),
    ));
}

fn init_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    spawned_players: Query<Entity, Added<Player>>,
) {
    for entity in &spawned_players {
        commands
            .entity(entity)
            .insert(PlayerSharedBundle::new(&mut meshes, &mut materials));
    }
}

#[derive(Component, Serialize, Deserialize)]
pub struct Player {
    pub client_id: u64,
}

#[derive(Component)]
pub struct PlayerControls;

#[derive(Bundle)]
struct PlayerSharedBundle {
    controls: PlayerControls,
    character_physics: CharacterPhysicsBundle,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    visibility: VisibilityBundle,
}

impl PlayerSharedBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> PlayerSharedBundle {
        Self {
            controls: PlayerControls,
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
            material: materials.add(Color::WHITE.into()),
            visibility: VisibilityBundle::default(),
        }
    }
}

#[derive(Debug, Default, Deserialize, Event, Serialize)]
struct ControlEvent(Vec3);

fn control(
    mut event: EventWriter<ControlEvent>,
    mut query: Query<(&PlayerControls, &mut CharacterVectors)>,
    input: Res<Input<KeyCode>>,
) {
    for (_controls, mut vectors) in &mut query {
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

        if vectors.velocity != Vec3::ZERO {
            event.send(ControlEvent(vectors.velocity));
        }
    }
}

fn control_handler(mut event: EventReader<FromClient<ControlEvent>>) {
    for FromClient { client_id, event } in event.read() {
        //info!("Client {:?} sent {:?}", client_id, event);
    }
}
