use bevy::prelude::*;
use shared::control::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, relay_client_authoritive_message::<GoTo>);
}
