use aeronet_replicon::client::*;
use aeronet_webtransport::client::*;
use bevy::{log::LogPlugin, prelude::*};
use bevy_console::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_replicon::prelude::*;

mod connect;
mod control;
mod game;
mod host;
mod lobby;
mod main_menu;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        DefaultPlugins.set(LogPlugin {
            custom_layer: custom_log_layer,
            filter: "duck_back=trace".to_string(),
            ..default()
        }),
        EguiPlugin::default(),
        WorldInspectorPlugin::new(),
        ConsolePlugin,
        WebTransportClientPlugin,
        RepliconPlugins,
        AeronetRepliconClientPlugin,
        control::plugin,
        lobby::plugin,
        shared::plugin,
        main_menu::plugin,
        game::plugin,
    ));
    app.insert_state(shared::AppState::MainMenu);
    app.add_observer(host::start_server);
    app.add_systems(Last, host::force_kill_server.run_if(on_message::<AppExit>));
    app.add_observer(connect::connect_client);
    app.add_observer(connect::on_connecting);
    app.add_observer(connect::on_connected);
    app.add_command_event_named("start_host", host::StartHostServer);
    app.add_command_event_named("connect", connect::ConnectClient);
}
