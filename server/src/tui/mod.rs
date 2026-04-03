#![allow(unused)]
use bevy::prelude::*;

mod events;
pub mod logging;
mod terminal;
mod ui;

pub fn plugin(app: &mut App) {
    app.add_plugins(events::register_messages_plugin);
    app.add_systems(Startup, terminal::start);
    app.add_systems(PreUpdate, events::capture_and_relay_events);
    app.add_systems(Update, exit_on_ctrl_c);
    app.add_systems(PostUpdate, (logging::update_history, ui::render).chain());
}

use crossterm::event::{KeyCode, KeyModifiers};
use events::KeyMessage;
pub fn exit_on_ctrl_c(
    mut app_exit: MessageWriter<AppExit>,
    mut key_messages: MessageReader<KeyMessage>,
) {
    for message in key_messages.read() {
        if message.modifiers == KeyModifiers::CONTROL && message.code == KeyCode::Char('c') {
            app_exit.write(AppExit::Success);
        }
    }
}
