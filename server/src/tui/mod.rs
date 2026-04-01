use bevy::prelude::*;

mod events;
pub mod logging;
mod terminal;
mod ui;

pub fn plugin(app: &mut App) {
    app.add_plugins(events::register_messages_plugin);
    app.add_systems(Startup, terminal::start);
    app.add_systems(PreUpdate, events::capture_and_relay_events);
    app.add_systems(PostUpdate, (logging::update_history, ui::render).chain());
}
