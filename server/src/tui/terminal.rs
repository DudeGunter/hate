use bevy::prelude::*;
use crossterm::{
    ExecutableCommand,
    terminal::{EnterAlternateScreen, enable_raw_mode},
};
use ratatui::{Terminal, prelude::*, try_restore};
use std::io::{Stdout, stdout};

#[derive(Resource, Deref, DerefMut)]
pub struct TerminalContext(Terminal<CrosstermBackend<Stdout>>);

impl Drop for TerminalContext {
    fn drop(&mut self) {
        match try_restore() {
            Err(error) => {
                error!("Failed to restore terminal: {}", error);
            }
            _ => {
                info!("Succefully restored terminal!");
            }
        }
    }
}

pub fn start(mut commands: Commands) {
    let mut stdout = stdout();

    match stdout.execute(EnterAlternateScreen) {
        Ok(_) => {
            let _ = enable_raw_mode();
            let backend = CrosstermBackend::new(stdout);

            if let Ok(terminal) = ratatui::Terminal::new(backend) {
                commands.insert_resource(TerminalContext(terminal));
            }
        }
        Err(error) => {
            error!("Failed to enter alternate terminal sceen: {}", error);
        }
    }
}
