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

pub struct PlayerInput {}

#[derive(Component, Reflect, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ColorDisplay;
