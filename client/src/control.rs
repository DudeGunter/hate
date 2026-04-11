use bevy::{
    app::{HierarchyPropagatePlugin, Propagate},
    prelude::*,
};
use shared::{
    AppState,
    management::{ClientOwns, GoTo, ownership::Owner},
};

pub fn plugin(app: &mut App) {
    app.add_plugins(HierarchyPropagatePlugin::<LocallyOwned, (), Owner>::new(
        PreUpdate,
    ));
    app.add_systems(Update, (recieve_goto, receive_client_owns));
}

pub fn recieve_goto(mut goto: MessageReader<GoTo>, mut game_state: ResMut<NextState<AppState>>) {
    for state in goto.read() {
        info!("Going to state {:?}", state.0);
        game_state.set(state.0);
    }
}

#[derive(Component, Reflect, PartialEq, Clone)]
pub struct LocallyOwned;

pub fn receive_client_owns(mut commands: Commands, mut client_owns: MessageReader<ClientOwns>) {
    for message in client_owns.read() {
        commands.entity(message.0).insert(Propagate(LocallyOwned));
    }
}
