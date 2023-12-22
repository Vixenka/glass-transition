use bevy::prelude::*;

use self::prototype_material::PrototypeMaterial;

pub mod cheat_menu;
pub mod prototype_material;
pub mod time;

pub struct DeveloperToolsPlugin;

impl Plugin for DeveloperToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<PrototypeMaterial>::default())
            .add_plugins(time::TimePlugin)
            .add_plugins(cheat_menu::CheatMenuPlugin);
    }
}
