//! Ex:
//! ```
//! pub fn update_player_color_display(
//!     mut commands: Commands,
//!     display: Single<Entity, With<PlayerColorDisplay>>,
//!     player_colors: Query<(Entity, &PlayerColor), Added<PlayerColor>>,
//! ) {
//!     for (player_id, player_color) in player_colors {
//!         let color_display = commands
//!             .spawn((
//!                 Node {
//!                     width: percent(100),
//!                     height: percent(100),
//!                     ..default()
//!                 },
//!                 BackgroundColor(player_color.0),
//!             ))
//!             .id();
//!         commands
//!             .entity(player_id) // The important part
//!             .add_one_related::<OwnedBy>(color_display);
//!         commands.entity(*display).add_child(color_display);
//!     }
//! }

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// linked_spawn makes the owned entities exist only if the owner exists.
// Esentially, when the entity with ```Owns``` is despawned, all entities in the ```Vec<Entity>```
// are also despawned.
// It is essentially a parent child relationship which doesn't move transforms or anything like that
// and is purely meant for networking, although it could be applied in other contexts.
#[derive(Component, Reflect)]
#[relationship_target(relationship = Owner, linked_spawn)]
pub struct Owns(Vec<Entity>);

#[derive(Component, Reflect, Serialize, Deserialize)]
#[relationship(relationship_target = Owns)]
pub struct Owner(#[entities] pub Entity);
