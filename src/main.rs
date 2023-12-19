use std::f32::consts::PI;

use bevy::{math::vec3, pbr::CascadeShadowConfigBuilder, prelude::*, render::camera::ScalingMode};
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GLASS TRANSITION".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(
            RapierPhysicsPlugin::<()>::default()
                .in_fixed_schedule()
                .with_physics_scale(1.0),
        )
        .add_plugins(RapierDebugRenderPlugin {
            enabled: true,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_characters)
        .add_systems(FixedUpdate, control_player)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(2.5, 1.0, 2.5),
        PbrBundle {
            mesh: meshes.add(shape::Box::new(5.0, 2.0, 5.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
    ));

    commands.spawn((
        PlayerBundle::new(),
        meshes.add(
            shape::Cylinder {
                radius: PlayerBundle::RADIUS,
                height: PlayerBundle::HALF_HEIGHT * 2.0,
                resolution: 16,
                segments: 1,
            }
            .into(),
        ),
        materials.add(Color::WHITE.into()),
        VisibilityBundle::default(),
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            PI / 2.,
            -PI / 4.,
        )),
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 7.0,
            maximum_distance: 25.0,
            ..default()
        }
        .into(),
        ..default()
    });
}

#[derive(Component)]
struct PlayerControls {}

#[derive(Component, Default)]
struct CharacterVectors {
    velocity: Vec3,
}

#[derive(Bundle)]
struct CharacterPhysicsBundle {
    rigid_body: RigidBody,
    controller: KinematicCharacterController,
    vectors: CharacterVectors,
}

#[derive(Bundle)]
struct PlayerBundle {
    transform: TransformBundle,
    controls: PlayerControls,
    character_physics: CharacterPhysicsBundle,
}

impl PlayerBundle {
    const RADIUS: f32 = 0.4;
    const HALF_HEIGHT: f32 = 0.4;

    pub fn new() -> PlayerBundle {
        Self {
            transform: TransformBundle::from_transform(Transform::from_xyz(0.0, 3.0, 0.0)),
            controls: PlayerControls {},
            character_physics: CharacterPhysicsBundle {
                rigid_body: RigidBody::KinematicVelocityBased,
                controller: KinematicCharacterController {
                    custom_shape: Some((
                        Collider::cylinder(Self::RADIUS, Self::HALF_HEIGHT),
                        Vect::ZERO,
                        Rot::IDENTITY,
                    )),
                    ..default()
                },
                vectors: CharacterVectors::default(),
            },
        }
    }
}

fn move_characters(mut query: Query<(&mut KinematicCharacterController, &CharacterVectors)>) {
    for (mut controller, vectors) in &mut query {
        controller.translation = Some(vectors.velocity);
    }
}

fn control_player(
    mut query: Query<(&PlayerControls, &mut CharacterVectors)>,
    input: Res<Input<KeyCode>>,
) {
    for (_controls, mut vectors) in &mut query {
        vectors.velocity += vec3(0.0, -0.005, 0.0);

        let speed = 0.01;

        if input.pressed(KeyCode::A) {
            vectors.velocity += vec3(-speed, 0.0, speed);
        }
        if input.pressed(KeyCode::S) {
            vectors.velocity += vec3(speed, 0.0, speed);
        }
        if input.pressed(KeyCode::D) {
            vectors.velocity += vec3(speed, 0.0, -speed);
        }
        if input.pressed(KeyCode::W) {
            vectors.velocity += vec3(-speed, 0.0, -speed);
        }

        let damping = 0.8;
        vectors.velocity.x *= damping;
        vectors.velocity.z *= damping;
    }
}
