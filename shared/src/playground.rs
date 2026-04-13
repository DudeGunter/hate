//! hard coded game scene.
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Reflect)]
pub struct Playground;

// server side collider scene
// this should be replaced with bsn!
pub fn collider_scene() -> impl Bundle {
    (
        Name::new("Playground"),
        Playground,
        Transform::default(),
        children![(
            Name::new("Floor"),
            Replicated,
            Transform::from_xyz(0.0, -10.0, 0.0),
            RigidBody::Static,
            Collider::cuboid(10.0, 0.1, 10.0),
        )],
    )
}
