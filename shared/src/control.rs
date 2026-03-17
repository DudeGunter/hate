use crate::GameState;
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
    app.replicate::<ControlAuthority>();
    app.add_client_message::<GoTo>(Channel::Ordered);
    app.add_server_message::<GoTo>(Channel::Ordered);
}

/// Any client with this can have control authority over the server
#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct ControlAuthority;

/// A client with ```ControlAuthority``` can send this to the server,
/// which is then relayed to all clients and the server
#[derive(Message, Serialize, Deserialize, Clone, Copy)]
pub struct GoTo(pub GameState);

// This should probably go into the server module.
pub fn relay_client_authoritive_message<'a, M: Message + Serialize + Deserialize<'a> + Clone>(
    mut messages: MessageReader<FromClient<M>>,
    mut to_clients: MessageWriter<ToClients<M>>,
    has_control: Query<Entity, With<ControlAuthority>>,
) {
    for message in messages.read() {
        if let Some(from) = message.client_id.entity()
            && has_control.contains(from)
        {
            to_clients.write(ToClients {
                mode: SendMode::Broadcast,
                message: message.message.clone(),
            });
        }
    }
}
