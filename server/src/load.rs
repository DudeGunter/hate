use aeronet::io::server::Server;
use bevy::prelude::*;
use shared::AppState;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Loading), || {
        info!("Loading lobby and assets")
    });
    app.add_systems(
        Update,
        go_to_lobby
            .run_if(server_is_up)
            .run_if(in_state(AppState::Loading)),
    );
}

pub fn server_is_up(query: Query<Entity, With<Server>>) -> bool {
    !query.is_empty()
}

pub fn go_to_lobby(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::Lobby);
}
