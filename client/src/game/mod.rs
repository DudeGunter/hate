use bevy::prelude::*;

pub fn plugin(app: &mut App) {}

#[derive(Resource)]
pub struct SelectedGameScene(Handle<Scene>);

pub fn spawn_basic_scene() {}
