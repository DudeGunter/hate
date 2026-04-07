use aeronet::io::Session;
use bevy::prelude::*;
use bevy_replicon::prelude::{FromClient, Replicated, SendMode, ToClients};
use shared::{
    GameState,
    management::scene::{
        AllFinishedLoading, FinishedLoading, GameScene, PleaseLoad, ReplicatedScenePath,
    },
};

pub fn plugin(app: &mut App) {
    app.add_sub_state::<GameState>();
    app.add_systems(
        OnEnter(GameState::Loading),
        spawn_replicated_scene_reference,
    );
    app.add_systems(
        Update,
        wait_on_response.run_if(in_state(GameState::Loading)),
    );
}

pub fn spawn_replicated_scene_reference(mut commands: Commands) {
    commands.spawn((
        Replicated,
        GameScene,
        // this is replicated once. it tell the client to not continue unless the scene is fully loaded
        PleaseLoad,
        // Which scene to replicate should be defined in the lobby probably.
        // why component? easy replication and multiple scenes *should* be possible
        ReplicatedScenePath("".to_string()),
    ));
}

pub fn wait_on_response(
    mut next_state: ResMut<NextState<GameState>>,
    mut messages: MessageReader<FromClient<FinishedLoading>>,
    mut write_finished: MessageWriter<ToClients<AllFinishedLoading>>,
    clients: Query<Entity, With<Session>>,
    mut n_messages_received: Local<usize>,
) {
    for _message in messages.read() {
        info!("A client finished loading all their scene!");
        *n_messages_received += 1;
    }

    if *n_messages_received == clients.count() {
        info!("All clients loaded! Starting game...");
        // we skip the waiting on others state because it functionally doesn't matter in this context
        next_state.set(GameState::Playing);
        write_finished.write(ToClients {
            mode: SendMode::Broadcast,
            message: AllFinishedLoading,
        });
    }
}
