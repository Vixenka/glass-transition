pub mod transform;

use bevy::prelude::*;

pub struct ReplicationPlugin;

impl Plugin for ReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(transform::TransformPlugin);
    }
}
