use aeronet::io::Session;
use aeronet_webtransport::server::*;
use bevy::{app::DynEq, prelude::*};
use bevy_replicon::prelude::*;
use shared::{
    GameState,
    management::{ControlAuthority, GoTo, SelectedGameScene},
    ownership::OwnedBy,
    player::*,
};
use std::time::SystemTime;

pub fn plugin(app: &mut App) {
    app.init_resource::<SelectedGameScene>();
    app.add_systems(OnEnter(GameState::Lobby), || {
        info!("Succesfully entered lobby.")
    });
    app.add_observer(on_connected);
    app.add_observer(on_session_request);
}

fn on_session_request(
    mut request: On<SessionRequest>,
    clients: Query<&ChildOf>,
    current_game_state: Res<State<GameState>>,
) {
    let client = request.event_target();
    let Ok(&ChildOf(server)) = clients.get(client) else {
        return;
    };

    info!("{client} connecting to {server} with headers:");
    for (header_key, header_value) in &request.headers {
        info!("  {header_key}: {header_value}");
    }
    match current_game_state.get() {
        GameState::Lobby => request.respond(SessionResponse::Accepted),
        _ => request.respond(SessionResponse::Forbidden),
    }
}

pub fn on_connected(
    trigger: On<Add, Session>,
    clients: Query<&ChildOf>,
    mut commands: Commands,
    mut goto: MessageWriter<ToClients<GoTo>>,
    current_game_state: Res<State<GameState>>,
) {
    // Only accept if in lobby
    if !current_game_state.get().dyn_eq(&GameState::Lobby) {
        return;
    }

    let client = trigger.event_target();
    let Ok(&ChildOf(server)) = clients.get(client) else {
        return;
    };
    info!("{client} connected to {server}");

    // generate a random-looking color
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("current system time should be after unix epoch")
        .as_millis();
    #[expect(
        clippy::cast_possible_truncation,
        reason = "truncation is what we want"
    )]
    let color = Color::srgb_u8((time * 3) as u8, (time * 5) as u8, (time * 7) as u8);

    let lobby_display = commands
        .spawn((PlayerColorDisplay, PlayerColor(color), Replicated))
        .id();

    commands
        .entity(client)
        .insert((Player, PlayerColor(color), ControlAuthority, Replicated))
        .add_one_related::<OwnedBy>(lobby_display);

    goto.write(ToClients {
        mode: SendMode::Direct(ClientId::Client(client)),
        message: GoTo(GameState::Lobby),
    });
}
