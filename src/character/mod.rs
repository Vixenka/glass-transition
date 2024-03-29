pub mod enemy;
pub mod player;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_replicon::replicon_core::replication_rules::{AppReplicationExt, Replication};
use serde::{Deserialize, Serialize};

use self::{enemy::EnemyPlugin, player::PlayerPlugin};

pub struct CharacterPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct MoveCharacters;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, EnemyPlugin))
            .replicate::<CharacterVectors>()
            .configure_sets(FixedUpdate, MoveCharacters.before(PhysicsSet::SyncBackend))
            .add_systems(
                FixedUpdate,
                (ground_characters, move_characters)
                    .chain()
                    .after(MoveCharacters)
                    .before(PhysicsSet::SyncBackend),
            );
    }
}

#[derive(Component, Clone, Default, Serialize, Deserialize)]
pub struct CharacterVectors {
    pub velocity: Vec3,
    pub damping_time: f32,
}

#[derive(Bundle)]
pub struct CharacterPhysicsBundle {
    pub rigid_body: RigidBody,
    pub controller: KinematicCharacterController,
    pub vectors: CharacterVectors,
    pub transform_interpolation: TransformInterpolation,
    replication: Replication,
}

impl CharacterPhysicsBundle {
    pub fn new(half_height: f32, radius: f32) -> Self {
        Self {
            rigid_body: RigidBody::KinematicVelocityBased,
            controller: KinematicCharacterController {
                offset: CharacterLength::Relative(0.1),
                custom_shape: Some((
                    Collider::cylinder(half_height, radius),
                    Vect::ZERO,
                    Rot::IDENTITY,
                )),
                apply_impulse_to_dynamic_bodies: true,
                ..default()
            },
            vectors: CharacterVectors::default(),
            transform_interpolation: TransformInterpolation::default(),
            replication: Replication,
        }
    }
}

pub fn ground_characters(
    mut query: Query<(&KinematicCharacterControllerOutput, &mut CharacterVectors)>,
) {
    for (controller_output, mut vectors) in &mut query {
        if controller_output.grounded {
            vectors.velocity.y = 0.0;
        }
    }
}

pub fn move_characters(mut query: Query<(&mut KinematicCharacterController, &CharacterVectors)>) {
    for (mut controller, vectors) in &mut query {
        controller.translation = Some(vectors.velocity);
    }
}
