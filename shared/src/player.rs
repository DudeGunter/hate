use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
    app.replicate::<Player>()
        .replicate::<Position>()
        .replicate::<PlayerColor>()
        .replicate::<PlayerColorDisplay>();
}

#[derive(Component, Reflect, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Player;

#[derive(Component, Reflect, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerColor(pub Color);

#[derive(Component, Reflect, Clone, Copy, Debug, Serialize, Deserialize)]
#[require(Transform)]
pub struct Position(pub Vec2);

pub struct PlayerInput {}

#[derive(Component, Reflect, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PlayerColorDisplay;
