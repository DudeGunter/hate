use bevy::prelude::*;
use bevy_replicon::prelude::*;
use shared::{
    AppState,
    management::{ControlAuthority, GoTo, PleaseGoTo},
};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, relay_client_authoritive_goto_gamestate_message);
}

pub fn relay_client_authoritive_goto_gamestate_message(
    mut from_clients: MessageReader<FromClient<PleaseGoTo>>,
    mut to_clients: MessageWriter<ToClients<GoTo>>,
    mut next_game_state: ResMut<NextState<AppState>>,
    has_control: Query<Entity, With<ControlAuthority>>,
) {
    for message in from_clients.read() {
        info!("Recieved a PleaseGoTo from client... checking authority.");
        if let Some(from) = message.client_id.entity()
            && has_control.contains(from)
        {
            info!("Client has authority! relaying and setting game state.");
            next_game_state.set(AppState::InGame);

            to_clients.write(ToClients {
                mode: SendMode::Broadcast,
                message: GoTo(message.0),
            });
        }
    }
}

/*
pub fn relay_client_authoritive_message<
    'a,
    M: Message + Serialize + Deserialize<'a> + Clone + std::fmt::Debug,
>(
    mut from_clients: MessageReader<FromClient<M>>,
    mut to_clients: MessageWriter<ToClients<M>>,
    has_control: Query<Entity, With<ControlAuthority>>,
) {
    for message in from_clients.read() {
        info!("Recieved message from client... attempting to relay.");
        if let Some(from) = message.client_id.entity()
            && has_control.contains(from)
        {
            info!("Relaying message {:?}", message.message);
            to_clients.write(ToClients {
                mode: SendMode::Broadcast,
                message: message.message.clone(),
            });
        }
    }
}
*/
