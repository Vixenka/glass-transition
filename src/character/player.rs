use bevy::{math::vec3, prelude::*};

use super::{CharacterPhysicsBundle, CharacterVectors};

pub const RADIUS: f32 = 0.4;
pub const HALF_HEIGHT: f32 = 0.4;

#[derive(Component)]
pub struct PlayerControls;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub transform: TransformBundle,
    pub controls: PlayerControls,
    pub character_physics: CharacterPhysicsBundle,
}

impl PlayerBundle {
    pub fn new(transform: Transform) -> PlayerBundle {
        Self {
            transform: TransformBundle::from_transform(transform),
            controls: PlayerControls,
            character_physics: CharacterPhysicsBundle::new(RADIUS, HALF_HEIGHT),
        }
    }
}

pub fn control(
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
    }
}
