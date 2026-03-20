use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};
use shared::control::{ControlAuthority, GoTo};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, relay_client_authoritive_message::<GoTo>);
}

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
