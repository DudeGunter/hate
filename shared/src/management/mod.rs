use crate::AppState;
use bevy::{ecs::entity::MapEntities, prelude::*};
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

pub mod ownership;
pub mod scene;

pub fn plugin(app: &mut App) {
    app.replicate::<ownership::Owner>();

    app.replicate::<ControlAuthority>();
    app.replicate::<scene::ReplicatedScenePath>();
    app.replicate::<scene::GameScene>();
    app.replicate_once::<scene::PleaseLoad>();

    app.add_client_message::<scene::FinishedLoading>(Channel::Unordered);
    app.add_server_message::<scene::AllFinishedLoading>(Channel::Unordered);

    app.add_client_message::<PleaseGoTo>(Channel::Ordered);
    app.add_server_message::<GoTo>(Channel::Ordered);

    app.add_mapped_server_message::<ClientOwns>(Channel::Ordered);
}

/// Any client with this can have control authority over the server
#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct ControlAuthority;

/// A client with ```ControlAuthority``` can send this to the server,
/// which is then relayed to all clients and the server as ```GoTo```
#[derive(Message, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct PleaseGoTo(pub AppState);

/// A client with ```ControlAuthority``` can send ```PleaseGoTo``` to the server,
/// which then relays this to all clients and the server
#[derive(Message, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct GoTo(pub AppState);

#[derive(Message, MapEntities, Serialize, Deserialize)]
pub struct ClientOwns(#[entities] pub Entity);
