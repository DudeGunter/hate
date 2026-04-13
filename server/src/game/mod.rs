use aeronet::io::Session;
use bevy::prelude::*;
use bevy_replicon::prelude::{FromClient, Replicated, SendMode, ToClients};
use shared::{
    GameState,
    management::scene::{
        AllFinishedLoading, FinishedLoading, GameScene, PleaseLoad, ReplicatedScenePath,
    },
};

mod player;

pub fn plugin(app: &mut App) {
    app.add_sub_state::<GameState>();
    app.add_plugins(player::plugin);
    app.add_systems(
        OnEnter(GameState::Loading),
        spawn_replicated_scene_reference,
    );

    // spawn playground
    app.add_systems(OnEnter(GameState::Loading), |mut commands: Commands| {
        commands.spawn(shared::playground::collider_scene());
    });

    app.add_systems(
        Update,
        wait_on_response.run_if(in_state(GameState::Loading)),
    );
}

#[derive(Resource, Reflect, Default)]
pub struct GameLoaded(pub bool);

pub fn spawn_replicated_scene_reference(mut commands: Commands) {
    commands.insert_resource(GameLoaded(true));
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
    game_loaded: Res<GameLoaded>,
    mut next_state: ResMut<NextState<GameState>>,
    mut messages: MessageReader<FromClient<FinishedLoading>>,
    mut write_finished: MessageWriter<ToClients<AllFinishedLoading>>,
    clients: Query<Entity, With<Session>>,
    mut n_messages_received: Local<usize>,
) {
    for _message in messages.read() {
        *n_messages_received += 1;
        info!(
            "A client has finished loading game assets... {}/{} are ready.",
            *n_messages_received,
            clients.count()
        );
    }

    if *n_messages_received == clients.count() && game_loaded.0 {
        info!("All clients loaded! Starting game...");
        // we skip the waiting on others state because it functionally doesn't matter in this context
        next_state.set(GameState::Playing);
        write_finished.write(ToClients {
            mode: SendMode::Broadcast,
            message: AllFinishedLoading,
        });
    }
}
