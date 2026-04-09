use aeronet::io::Session;
use bevy::prelude::*;
use bevy_replicon::prelude::{FromClient, Replicated, SendMode, ToClients};
use shared::{
    GameState,
    management::{
        ownership::OwnedBy,
        scene::{AllFinishedLoading, FinishedLoading, GameScene, PleaseLoad, ReplicatedScenePath},
    },
    player::{Player, Position},
};

pub fn plugin(app: &mut App) {
    app.add_sub_state::<GameState>();
    app.add_systems(
        OnEnter(GameState::Loading),
        (spawn_basic_2d_scene, spawn_replicated_scene_reference),
    );
    app.add_systems(
        Update,
        wait_on_response.run_if(in_state(GameState::Loading)),
    );

    app.add_systems(
        Update,
        randomly_change_positions.run_if(in_state(GameState::Playing)),
    );
}

#[derive(Resource, Reflect, Default)]
pub struct GameLoaded(pub bool);

pub fn spawn_basic_2d_scene(mut commands: Commands, sessions: Query<Entity, With<Session>>) {
    commands.insert_resource(GameLoaded(true));
    for entity in sessions {
        let character = commands
            .spawn((Player, Position(Vec2::ZERO), Replicated))
            .id();
        commands
            .entity(entity)
            .add_one_related::<OwnedBy>(character);
    }
}

pub fn randomly_change_positions(player_positions: Query<&mut Position, With<Player>>) {
    for mut position in player_positions {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        match seed % 2 {
            0 => position.0.x += 1.0,
            _ => position.0.x -= 1.0,
        }
    }
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
