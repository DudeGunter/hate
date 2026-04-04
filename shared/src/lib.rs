use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod consts;
pub mod management;
pub mod player;

pub fn plugin(app: &mut App) {
    app.add_plugins((player::plugin, management::plugin));
}

// client and server insert the state independently
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameState {
    MainMenu,
    Loading,
    Lobby,
    Playing,
}

#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[source(GameState = GameState::Playing)]
pub enum PlayingState {
    #[default]
    Loading,
    WaitingOnOthers,
    Playing,
    Exiting,
}
