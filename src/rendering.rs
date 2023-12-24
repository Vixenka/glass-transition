use bevy::prelude::*;

use self::sprite::SpritePlugin;

pub mod sprite;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpritePlugin);
    }
}
