use bevy::prelude::*;
use bevy_replicon::prelude::Replicated;
use shared::{PlayingState, management::scene::ReplicatedScenePath};

pub fn plugin(app: &mut App) {
    app.add_sub_state::<PlayingState>();
    app.add_systems(
        OnEnter(PlayingState::Loading),
        spawn_replicated_scene_reference,
    );
}

pub fn spawn_replicated_scene_reference(mut commands: Commands) {
    commands.spawn((Replicated, ReplicatedScenePath("".to_string())));
}
