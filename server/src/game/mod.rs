use bevy::prelude::*;
use shared::{PlayingState, management::SelectedGameScene};

pub fn plugin(app: &mut App) {
    app.add_sub_state::<PlayingState>();
    app.add_systems(OnEnter(PlayingState::Loading), load_game_scene);
}

pub fn load_game_scene(mut selected_scene: ResMut<SelectedGameScene>) {
    if selected_scene.is_none() {
        selected_scene.select("default.scn");
    }
}
