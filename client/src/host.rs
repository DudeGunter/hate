use bevy::{prelude::*, tasks::IoTaskPool};
use bevy_console::prelude::*;
use duck_back::Else;
use shared::consts::LET_HOST_KNOW_KEY;
use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use crate::connect::ConnectClient;

pub fn plugin(app: &mut App) {
    app.add_observer(start_server);
    app.add_systems(Update, watch_for_lobby_bool);
    app.add_systems(Last, force_kill_server.run_if(on_message::<AppExit>));
}

#[derive(Event, Component, Clone)]
pub struct StartHostServer;

#[derive(Component)]
pub struct ServerProcess {
    pub child: Child,
    lobby_open: Arc<AtomicBool>,
}

#[derive(Component, Reflect)]
pub struct LobbyOpen;

pub fn start_server(_on: On<StartHostServer>, mut commands: Commands) {
    let mut child = Command::new("bevy")
        .args(["run", "--features", "server"])
        .stderr(Stdio::piped())
        .spawn()
        .else_error()?;

    let lobby_open = Arc::new(AtomicBool::new(false));
    let cloned_lobby_open = lobby_open.clone();

    // Pipe server outputs
    let stderr = child.stderr.take().else_error()?;
    IoTaskPool::get()
        .spawn(async move {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        let clean = match line.rfind("\x1b[0m") {
                            Some(pos) => line[pos + 4..].trim_start(),
                            None => line.trim_start(),
                        };
                        if line.find(LET_HOST_KNOW_KEY).is_some() {
                            cloned_lobby_open.store(true, Ordering::Relaxed);
                        }
                        simple!("[server] {}", clean);
                    }
                    Err(e) => simple!("[server error] {}", e),
                }
            }
        })
        .detach();

    // When host is despawned the server is killed.
    // This should be handled in a different way
    // A client needs control and comms over server
    commands.spawn(ServerProcess { child, lobby_open });
}

pub fn watch_for_lobby_bool(
    mut commands: Commands,
    host: Single<(Entity, &ServerProcess), Without<LobbyOpen>>,
) {
    let (entity, process) = *host;
    if process.lobby_open.load(Ordering::Relaxed) {
        commands.entity(entity).insert(LobbyOpen);
        commands.trigger(ConnectClient);
    }
}

pub fn force_kill_server(mut host: Single<&mut ServerProcess>) {
    let _ = host.child.kill();
}
