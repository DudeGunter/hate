use aeronet::io::server::Server;
use aeronet_replicon::server::{AeronetRepliconServer, AeronetRepliconServerPlugin};
use aeronet_webtransport::{cert, server::*, wtransport};
use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*, state::app::StatesPlugin};
use bevy_replicon::prelude::*;
use shared::{
    AppState,
    consts::{SERVER_PORT, TICK_RATE},
};
use std::time::Duration;

mod control;
mod game;
mod lobby;
#[cfg(feature = "tui")]
mod tui;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        LogPlugin {
            #[cfg(feature = "tui")]
            custom_layer: tui::logging::custom_layer,
            filter: "duck_back=trace".to_string(),
            ..default()
        },
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / f64::from(TICK_RATE),
        ))),
        StatesPlugin,
        WebTransportServerPlugin,
        RepliconPlugins,
        AeronetRepliconServerPlugin,
        #[cfg(feature = "tui")]
        tui::plugin,
        control::plugin,
        shared::plugin,
        lobby::plugin,
        game::plugin,
    ));
    app.insert_state(AppState::Loading);
    app.add_systems(Startup, open_web_transport_server);
    app.add_observer(go_to_lobby);
}

fn open_web_transport_server(mut commands: Commands) {
    let identity = wtransport::Identity::self_signed(["localhost", "127.0.0.1", "::1"])
        .expect("all given SANs should be valid DNS names");
    let cert = &identity.certificate_chain().as_slice()[0];
    let spki_fingerprint = cert::spki_fingerprint_b64(cert).expect("should be a valid certificate");
    let cert_hash = cert::hash_to_b64(cert.hash());
    info!("************************");
    info!("SPKI FINGERPRINT");
    info!("  {spki_fingerprint}");
    info!("CERTIFICATE HASH");
    info!("  {cert_hash}");
    info!("************************");

    let config = ServerConfig::builder()
        .with_bind_default(SERVER_PORT)
        .with_identity(identity)
        .keep_alive_interval(Some(Duration::from_secs(1)))
        .max_idle_timeout(Some(Duration::from_secs(5)))
        .expect("should be a valid idle timeout")
        .build();
    let server = commands
        .spawn((
            Name::new("WebTransport Server"),
            // IMPORTANT
            //
            // Make sure to insert this component into your server entity,
            // so that `aeronet_replicon` knows you want to use this for `bevy_replicon`!
            AeronetRepliconServer,
        ))
        .queue(WebTransportServer::open(config))
        .id();
    info!("Opening WebTransport server {server}");
}

pub fn go_to_lobby(_trigger: On<Add, Server>, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Lobby);
}
