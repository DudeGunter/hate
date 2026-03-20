use bevy::prelude::*;

#[derive(Component, Reflect)]
#[relationship(relationship_target = Owns)]
pub struct OwnedBy(pub Entity);

#[derive(Component, Reflect)]
#[relationship_target(relationship = OwnedBy, linked_spawn)]
pub struct Owns(Vec<Entity>);
