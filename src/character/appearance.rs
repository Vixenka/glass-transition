use bevy::{
    math::vec2,
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology, texture::ImageLoaderSettings},
};

use crate::rendering::sprite::{SpriteMaterial, SpriteMaterialUniforms, StandardMaterialSprite};

pub struct CharacterAppearancePlugin;

impl Plugin for CharacterAppearancePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Debug, Resource)]
pub struct CharacterAppearanceAssets {
    pub plane_mesh: Handle<Mesh>,

    pub player_material: Handle<StandardMaterialSprite>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct RotateTowardsCamera;

#[derive(Debug, Bundle)]
pub struct CharacterAppearanceBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterialSprite>,
    pub visibility: VisibilityBundle,
    pub rotate_towards_camera: RotateTowardsCamera,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterialSprite>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(CharacterAppearanceAssets {
        plane_mesh: meshes.add(
            Mesh::new(PrimitiveTopology::TriangleList)
                .with_inserted_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vec![
                        [-0.5, -0.5, 0.0],
                        [-0.5, 0.5, 0.0],
                        [0.5, -0.5, 0.0],
                        [0.5, 0.5, 0.0],
                    ],
                )
                .with_inserted_attribute(
                    Mesh::ATTRIBUTE_UV_0,
                    vec![[0.0, 1.0], [0.0, 0.0], [1.0, 1.0], [1.0, 0.0]],
                )
                .with_inserted_attribute(
                    Mesh::ATTRIBUTE_NORMAL,
                    vec![
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0],
                    ],
                )
                .with_indices(Some(Indices::U32(vec![2, 1, 0, 1, 2, 3]))),
        ),

        player_material: materials.add(StandardMaterialSprite {
            base: StandardMaterial {
                base_color_texture: Some(asset_server.load_with_settings(
                    "textures/character/player.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.is_srgb = true;
                    },
                )),
                alpha_mode: AlphaMode::Mask(0.5),
                ..default()
            },
            extension: SpriteMaterial {
                uniforms: SpriteMaterialUniforms {
                    billboard_size: vec2(1.0, 1.0),
                },
            },
        }),
    });
}
