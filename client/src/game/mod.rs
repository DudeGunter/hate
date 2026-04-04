use bevy::prelude::*;

pub fn plugin(app: &mut App) {}

#[derive(Resource)]
pub struct SelectedGameScene(Option<String>);

pub fn spawn_basic_scene() {}
