use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use random_color::{Luminosity, RandomColor};

const SHADER_PATH: &'static str = "shaders/developer_tools/prototype_material.wgsl";

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct PrototypeMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    base_texture: Handle<Image>,
}

impl PrototypeMaterial {
    /// Returns a handle to prototype material with a random color based on the feature name.
    /// # Arguments
    /// * `materials` - Collection of materials.
    /// * `assets` - Asset server.
    pub fn get(
        materials: &mut ResMut<Assets<PrototypeMaterial>>,
        assets: &Res<AssetServer>,
        feature_name: &str,
    ) -> Handle<PrototypeMaterial> {
        let mut hasher = DefaultHasher::new();
        feature_name.hash(&mut hasher);
        let hash = hasher.finish();

        let rgb = RandomColor::new()
            .luminosity(Luminosity::Bright)
            .seed(hash)
            .to_rgb_array();

        materials.add(PrototypeMaterial {
            color: Color::rgb_u8(rgb[0], rgb[1], rgb[2]),
            base_texture: assets.load("textures/developer_tools/prototype.png"),
        })
    }
}

impl Material for PrototypeMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
}
