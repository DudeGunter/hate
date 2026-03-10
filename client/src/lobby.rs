use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::PrimaryEguiContext;
use shared::GameState;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Lobby), spawn_lobby);
}

pub fn spawn_lobby(mut commands: Commands) {
    commands.spawn((Camera2d, PrimaryEguiContext));
}
