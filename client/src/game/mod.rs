use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::PrimaryEguiContext;
use shared::{AppState, GameState};

mod load;

pub fn plugin(app: &mut App) {
    app.add_sub_state::<GameState>();
    app.add_systems(OnEnter(AppState::InGame), load::manage_replicated_scene);
    app.add_systems(
        Update,
        load::check_please_load_scenes.run_if(in_state(GameState::Loading)),
    );
    app.add_systems(
        Update,
        load::wait_on_server_and_others.run_if(in_state(GameState::WaitingOnOthers)),
    );

    app.add_systems(OnEnter(GameState::Playing), say_hi);
}

pub fn say_hi(mut commands: Commands) {
    info!("We have entered the main game state...");
    commands.spawn((Camera2d, Text::new("hello"), PrimaryEguiContext));
}
