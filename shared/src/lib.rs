use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod consts;
pub mod management;
pub mod physics;
pub mod player;

pub fn plugin(app: &mut App) {
    app.add_plugins((player::plugin, management::plugin));
}

// client and server insert the state independently
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppState {
    MainMenu,
    Loading,
    Lobby,
    InGame,
}

#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[source(AppState = AppState::InGame)]
pub enum GameState {
    #[default]
    Loading,
    WaitingOnOthers,
    Playing,
    Exiting,
}
