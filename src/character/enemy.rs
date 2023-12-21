use bevy::prelude::*;
use bevy_replicon::replicon_core::replication_rules::AppReplicationExt;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

use super::{player, CharacterPhysicsBundle};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<Enemy>()
            .add_systems(PostUpdate, init_enemies);
    }
}

pub fn spawn(commands: &mut Commands, enemy: Enemy, transform: Transform) {
    commands.spawn((enemy, transform));
}

#[derive(Clone, Component, Deserialize, Serialize)]
pub struct Enemy {
    pub kind: EnemyKind,
}

#[derive(Clone, Copy, Deserialize, Serialize, Sequence, Debug)]
pub enum EnemyKind {
    Dummy,
}

fn init_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    spawned: Query<(Entity, &Enemy), Added<Enemy>>,
) {
    for (entity, enemy) in &spawned {
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
            materials.add(
                match enemy.kind {
                    EnemyKind::Dummy => Color::RED,
                }
                .into(),
            ),
            VisibilityBundle::default(),
        ));
    }
}
