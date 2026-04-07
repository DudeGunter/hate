use bevy::prelude::*;
use shared::{
    GameState,
    management::scene::{
        AllFinishedLoading, FinishedLoading, GameScene, PleaseLoad, ReplicatedScenePath,
    },
};

#[allow(unused)]
pub fn manage_replicated_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scenes: Query<(Entity, &ReplicatedScenePath), Added<GameScene>>,
) {
    for (entity, path) in scenes {
        let mut entity_cmds = commands.entity(entity);
        // Temparary until we get some real scene assets. Grotesque fix
        entity_cmds.remove::<PleaseLoad>();
        continue;
        // Buisness as usual after this
        // I think I've mentioned this somewhere else in the docs probably.
        // This should instead of being a path to one scene, be a collection of
        // assets or have it be just a folder describing the level.
        // The folder could then have a desc.toml or something.
        // If the asset fails to load, tell the server and ask for web link.
        // Aka https://....zip
        let potential_scene: Handle<Scene> = asset_server.load(path.0.clone());
        entity_cmds.insert(SceneRoot(potential_scene));
    }
}

// WARNING!!! this logic is pretty much untested
pub fn check_please_load_scenes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
    mut finished_loading: MessageWriter<FinishedLoading>,
    scenes: Query<(Entity, &SceneRoot), With<PleaseLoad>>,
) {
    for (entity, scene) in scenes {
        if let Some(status) = asset_server.get_load_state(scene.0.id()) {
            use bevy::asset::LoadState as S;
            match status {
                S::Loaded => {
                    commands.entity(entity).remove::<PleaseLoad>();
                }
                S::Loading => {} // keep this out here because there could be application and I want to be reminded
                S::NotLoaded => {}
                S::Failed(error) => {
                    error!("Failed to load scene: {}", error);
                }
            }
        }
    }
    // if there are no assets to load,
    if scenes.count() == 0 {
        // Tell the server I'm ready and wait on the others/go to waiting state
        info!("Finished loading all required assets, telling server and waiting for others.");
        finished_loading.write(FinishedLoading);
        next_state.set(GameState::WaitingOnOthers);
    }
}

pub fn wait_on_server_and_others(
    mut next_state: ResMut<NextState<GameState>>,
    message: MessageReader<AllFinishedLoading>,
) {
    if !message.is_empty() {
        info!("All clients finished loading, moving to playing state.");
        next_state.set(GameState::Playing);
    }
}
