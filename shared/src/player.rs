use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
    app.replicate::<Player>()
        .replicate::<PlayerColor>()
        .replicate::<ColorDisplay>();
}

#[derive(Component, Reflect, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Player;

#[derive(Component, Reflect, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerColor(pub Color);

#[derive(Component, Reflect, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ColorDisplay;

pub mod actions {
    //! player actions.
    //! I think bei may be a very good candidate here
    //! more thoughts...
    pub struct Action;

    pub struct Jump;

    pub struct Move;
}
