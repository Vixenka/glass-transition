use bevy::{prelude::*, ptr::Ptr};
use bevy_replicon::{bincode, prelude::*, replicon_core::replication_rules};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use crate::{math::lerp_exponent_in_time, network::MAX_TICK_RATE};

pub struct TransformPlugin;

impl Plugin for TransformPlugin {
    fn build(&self, app: &mut App) {
        app.replicate_with::<Transform>(
            serialize_transform,
            deserialize_transform,
            replication_rules::remove_component::<Transform>,
        )
        .add_systems(PreUpdate, apply_synced_transform);
    }
}

#[derive(Clone, Serialize, Deserialize, Component)]
pub struct SyncedTransform {
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
}

impl From<Transform> for SyncedTransform {
    fn from(value: Transform) -> Self {
        Self {
            translation: value.translation,
            rotation: value.rotation,
            scale: value.scale,
        }
    }
}

impl From<SyncedTransform> for Transform {
    fn from(value: SyncedTransform) -> Self {
        Self {
            translation: value.translation,
            rotation: value.rotation,
            scale: value.scale,
        }
    }
}

fn apply_synced_transform(mut query: Query<(&mut Transform, &SyncedTransform)>, time: Res<Time>) {
    let t = lerp_exponent_in_time(1.0 / MAX_TICK_RATE as f32, 0.0001, time.delta_seconds());

    for (mut transform, synced_transform) in query.iter_mut() {
        transform.translation = transform.translation.lerp(synced_transform.translation, t);
        transform.rotation = transform.rotation.lerp(synced_transform.rotation, t);
        transform.scale = synced_transform.scale;
    }
}

fn serialize_transform(component: Ptr, cursor: &mut Cursor<Vec<u8>>) -> bincode::Result<()> {
    // SAFETY: Function called for registered `ComponentId`.
    let transform: &Transform = unsafe { component.deref() };
    bincode::serialize_into(cursor, &SyncedTransform::from(*transform))
}

fn deserialize_transform(
    entity: &mut EntityWorldMut,
    _entity_map: &mut ServerEntityMap,
    cursor: &mut Cursor<&[u8]>,
    _replicon_tick: RepliconTick,
) -> bincode::Result<()> {
    let transform: SyncedTransform = bincode::deserialize_from(cursor)?;
    if entity.get::<Transform>().is_none() {
        entity.insert(Transform::from(transform.clone()));
    }
    entity.insert(transform);

    Ok(())
}
