use bevy::prelude::*;
use shared::GameState;

mod load;

pub fn plugin(app: &mut App) {
    app.add_sub_state::<GameState>();
    app.add_systems(OnEnter(GameState::Loading), load::manage_replicated_scene);
    app.add_systems(
        Update,
        load::check_please_load_scenes.run_if(in_state(GameState::Loading)),
    );
    app.add_systems(
        Update,
        load::wait_on_server_and_others.run_if(in_state(GameState::WaitingOnOthers)),
    );
}
