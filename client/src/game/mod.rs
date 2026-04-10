use bevy::prelude::*;
use shared::{AppState, GameState};

mod load;
mod player;

pub fn plugin(app: &mut App) {
    app.add_sub_state::<GameState>();
    app.add_plugins(player::plugin);
    app.add_systems(OnEnter(AppState::InGame), load::manage_replicated_scene);
    app.add_systems(
        Update,
        load::check_please_load_scenes.run_if(in_state(GameState::Loading)),
    );
    app.add_systems(
        Update,
        load::wait_on_server_and_others.run_if(in_state(GameState::WaitingOnOthers)),
    );
}
