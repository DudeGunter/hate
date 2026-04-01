use bevy::prelude::*;

mod events;
mod terminal;
mod ui;

pub fn plugin(app: &mut App) {
    app.add_plugins(events::register_messages_plugin);
    app.add_systems(Startup, terminal::start);
    app.add_systems(PreUpdate, events::capture_and_relay_events);
    app.add_systems(PostUpdate, ui::render);
}
