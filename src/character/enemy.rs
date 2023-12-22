use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_replicon::replicon_core::replication_rules::AppReplicationExt;
use serde::{Deserialize, Serialize};

use super::{player, CharacterPhysicsBundle};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<Enemy>()
            .replicate::<DummyEnemy>()
            .add_systems(PostUpdate, init_enemies);
    }
}

pub fn spawn<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    enemy: Enemy,
    transform: Transform,
) -> EntityCommands<'w, 's, 'a> {
    commands.spawn((enemy, transform))
}

#[derive(Clone, Component, Deserialize, Serialize)]
pub struct Enemy;

#[derive(Clone, Component, Deserialize, Serialize)]
pub struct DummyEnemy;

fn init_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    spawned: Query<Entity, Added<Enemy>>,
) {
    for entity in &spawned {
        commands.entity(entity).insert((
            GlobalTransform::IDENTITY,
            CharacterPhysicsBundle::new(player::RADIUS, player::HALF_HEIGHT),
            meshes.add(
                shape::Cylinder {
                    radius: player::RADIUS,
                    height: player::HALF_HEIGHT * 2.0,
                    resolution: 16,
                    segments: 1,
                }
                .into(),
            ),
            materials.add(Color::RED.into()),
            VisibilityBundle::default(),
        ));
    }
}
