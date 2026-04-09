use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_replicon::prelude::*;

pub fn plugin(app: &mut App) {
    app.replicate::<Position>();
    app.replicate::<Rotation>();
    app.replicate::<Collider>();
    app.replicate::<RigidBody>();
    app.replicate::<LinearVelocity>();
    app.replicate::<AngularVelocity>();
}
