pub mod developer_tools;

use bevy::prelude::*;

#[bevy_main]
fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
