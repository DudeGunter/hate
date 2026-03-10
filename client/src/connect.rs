use aeronet::io::{Session, SessionEndpoint};
use aeronet_replicon::client::AeronetRepliconClient;
use aeronet_webtransport::{cert, client::*};
use bevy::prelude::*;

#[derive(Event, Component, Clone, Reflect, Debug)]
pub struct ConnectClient;

pub fn connect_client(_on: On<ConnectClient>, mut commands: Commands) {
    let config = web_transport_config(String::new());
    commands.spawn_empty().queue(WebTransportClient::connect(
        config,
        format!("https://127.0.0.1:8080"),
    ));
}

pub fn on_connecting(trigger: On<Add, SessionEndpoint>, mut commands: Commands) {
    commands
        .entity(trigger.event_target())
        .insert(AeronetRepliconClient);
}

pub fn on_connected(_trigger: On<Add, Session>) {
    info!("Succesfully connected!");
}

fn web_transport_config(cert_hash: String) -> ClientConfig {
    use {aeronet_webtransport::wtransport::tls::Sha256Digest, core::time::Duration};

    let config = ClientConfig::builder().with_bind_default();

    let config = if cert_hash.is_empty() {
        warn!("Connecting without certificate validation");
        config.with_no_cert_validation()
    } else {
        match cert::hash_from_b64(&cert_hash) {
            Ok(hash) => config.with_server_certificate_hashes([Sha256Digest::new(hash)]),
            Err(err) => {
                warn!("Failed to read certificate hash from string: {err:?}");
                config.with_server_certificate_hashes([])
            }
        }
    };

    config
        .keep_alive_interval(Some(Duration::from_secs(1)))
        .max_idle_timeout(Some(Duration::from_secs(5)))
        .expect("should be a valid idle timeout")
        .build()
}
