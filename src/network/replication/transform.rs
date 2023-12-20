use bevy::{prelude::*, ptr::Ptr};
use bevy_replicon::{bincode, prelude::*, replicon_core::replication_rules};
use std::io::Cursor;

pub struct TransformPlugin;

impl Plugin for TransformPlugin {
    fn build(&self, app: &mut App) {
        app.replicate_with::<Transform>(
            serialize_transform,
            deserialize_transform,
            replication_rules::remove_component::<Transform>,
        );
    }
}

fn serialize_transform(component: Ptr, cursor: &mut Cursor<Vec<u8>>) -> bincode::Result<()> {
    // SAFETY: Function called for registered `ComponentId`.
    let transform: &Transform = unsafe { component.deref() };
    bincode::serialize_into(cursor, &transform.translation)
}

fn deserialize_transform(
    entity: &mut EntityWorldMut,
    _entity_map: &mut ServerEntityMap,
    cursor: &mut Cursor<&[u8]>,
    _replicon_tick: RepliconTick,
) -> bincode::Result<()> {
    let translation: Vec3 = bincode::deserialize_from(cursor)?;
    entity.insert(TransformBundle::from_transform(
        Transform::from_translation(translation),
    ));

    Ok(())
}
