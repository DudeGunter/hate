use crate::GameState;
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
    app.replicate::<ControlAuthority>();
    app.add_server_message::<GoTo>(Channel::Ordered);
    app.add_client_message::<GoTo>(Channel::Ordered);
}

/// Any client with this can have control authority over the server
#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct ControlAuthority;

/// A client with ```ControlAuthority``` can send this to the server,
/// which is then relayed to all clients and the server
#[derive(Message, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct GoTo(pub GameState);
