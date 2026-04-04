use crate::GameState;
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub fn plugin(app: &mut App) {
    app.replicate::<ControlAuthority>();

    app.add_client_message::<PleaseGoTo>(Channel::Ordered);
    app.add_server_message::<GoTo>(Channel::Ordered);

    app.add_client_message::<SelectGameScene>(Channel::Ordered);
    app.add_server_message::<Response<SelectGameScene>>(Channel::Ordered);
}

/// Any client with this can have control authority over the server
#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct ControlAuthority;

/// A client with ```ControlAuthority``` can send this to the server,
/// which is then relayed to all clients and the server as ```GoTo```
#[derive(Message, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct PleaseGoTo(pub GameState);

/// A client with ```ControlAuthority``` can send ```PleaseGoTo``` to the server,
/// which then relays this to all clients and the server
#[derive(Message, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct GoTo(pub GameState);

// String path to scene asset
#[derive(Resource, Reflect)]
pub struct SelectedGameScene(Option<String>);

// Sent to server
#[derive(Message, Serialize, Deserialize)]
pub struct SelectGameScene(String);

#[derive(Message, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum Response<T> {
    Success(PhantomData<T>),
    Fail(PhantomData<T>),
}
