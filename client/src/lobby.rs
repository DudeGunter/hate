use crate::main_menu::{text, trigger_event_on_button_pressed};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::PrimaryEguiContext;
use shared::{
    GameState,
    control::GoTo,
    ownership::*,
    player::{Player, PlayerColor},
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Lobby), (spawn_lobby, spawn_ui));
    app.add_systems(
        Update,
        (
            update_player_count,
            update_player_color_display,
            trigger_event_on_button_pressed::<StartGame>,
        )
            .run_if(in_state(GameState::Lobby)),
    );
    app.add_observer(start_game);
}

pub fn spawn_lobby(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        PrimaryEguiContext,
        DespawnOnExit(GameState::Lobby),
    ));
}

pub fn spawn_ui(mut commands: Commands) {
    commands.spawn((DespawnOnExit(GameState::Lobby), PlayerCount));

    commands.spawn((
        DespawnOnExit(GameState::Lobby),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        children![(text("Start Game"), StartGame)],
    ));

    commands.spawn((
        DespawnOnExit(GameState::Lobby),
        PlayerColorDisplay,
        ZIndex(-1),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
    ));
}

#[derive(Component, Reflect)]
pub struct LobbyMenu;

#[derive(Component, Event, Reflect, Clone)]
#[require(Button)]
pub struct StartGame;

pub fn start_game(_trigger: On<StartGame>, mut goto: MessageWriter<GoTo>) {
    info!("Sending GoTo(GameState::Playing) to server.");
    goto.write(GoTo(GameState::Playing));
}

#[derive(Component, Reflect)]
pub struct PlayerColorDisplay;

#[derive(Component, Reflect)]
#[require(Text)]
pub struct PlayerCount;

pub fn update_player_count(
    mut text: Single<&mut Text, With<PlayerCount>>,
    n_clients: Query<&Player>,
) {
    text.0 = format!("{} players connected.", n_clients.count());
}

pub fn update_player_color_display(
    mut commands: Commands,
    display: Single<Entity, With<PlayerColorDisplay>>,
    player_colors: Query<(Entity, &PlayerColor), Added<PlayerColor>>,
) {
    for (player_id, player_color) in player_colors {
        let color_display = commands
            .spawn((
                Node {
                    width: percent(100),
                    height: percent(100),
                    ..default()
                },
                BackgroundColor(player_color.0),
            ))
            .id();
        commands
            .entity(player_id)
            .add_one_related::<OwnedBy>(color_display);
        commands.entity(*display).add_child(color_display);
    }
}
