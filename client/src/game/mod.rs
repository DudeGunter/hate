use bevy::prelude::*;
use shared::{
    PlayingState,
    management::scene::{GameScene, ReplicatedScenePath},
};

pub fn plugin(app: &mut App) {
    app.add_sub_state::<PlayingState>();
    app.add_systems(OnEnter(PlayingState::Loading), manage_replicated_scene);
}

pub fn manage_replicated_scene(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    scenes: Query<(Entity, &ReplicatedScenePath), With<GameScene>>,
) {
    for (entity, path) in scenes {
        let mut entity_cmds = commands.entity(entity);
        let potential_scene: Handle<Scene> = asset_server.load(path.0.clone());
        entity_cmds.insert(SceneRoot(potential_scene));
    }
}
