use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod consts;
pub mod control;
pub mod ownership;
pub mod player;

pub fn plugin(app: &mut App) {
    app.add_plugins((player::plugin, control::plugin));
}

// client and server insert the state independently
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Serialize, Deserialize)]
pub enum GameState {
    MainMenu,
    Loading,
    Lobby,
    Playing,
}
