pub mod player;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component, Default)]
pub struct CharacterVectors {
    pub velocity: Vec3,
}

#[derive(Bundle)]
pub struct CharacterPhysicsBundle {
    pub rigid_body: RigidBody,
    pub controller: KinematicCharacterController,
    pub vectors: CharacterVectors,
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
                ..default()
            },
            vectors: CharacterVectors::default(),
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
