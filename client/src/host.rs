use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use bevy_console::prelude::*;
use duck_back::Else;
use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
};

#[derive(Event, Component, Clone)]
pub struct StartHostServer;

#[allow(unused)]
#[derive(Component)]
pub struct Host(pub Child);

// Kill server when host is dropped
// It shouldn't be done like this,
// I have this so when client crashes, server doesn't run without me knowing
impl Drop for Host {
    fn drop(&mut self) {
        let _ = self.0.kill();
    }
}

pub fn start_server(_on: On<StartHostServer>, mut commands: Commands) {
    let mut child = Command::new("bevy")
        .args(["run", "--features", "server"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .else_error()?;

    // Pipe server outputs
    let stderr = child.stderr.take().else_error()?;
    AsyncComputeTaskPool::get()
        .spawn(async move {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                match line {
                    Ok(l) => simple!("[server] {}", l),
                    Err(e) => simple!("[server error] {}", e),
                }
            }
        })
        .detach();
    let stdout = child.stdout.take().else_error()?;
    AsyncComputeTaskPool::get()
        .spawn(async move {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(l) => simple!("[server] {}", l),
                    Err(e) => simple!("[server error] {}", e),
                }
            }
        })
        .detach();

    // When host is despawned the server is killed.
    // This should be handled in a different way
    // A client needs control and comms over server
    commands.spawn(Host(child));
}
