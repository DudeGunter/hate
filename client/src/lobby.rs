use crate::main_menu::{text, trigger_event_on_button_pressed};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::PrimaryEguiContext;
use shared::{
    AppState,
    management::PleaseGoTo,
    player::{ColorDisplay, Player, PlayerColor},
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Lobby), (spawn_lobby, spawn_ui));
    app.add_systems(
        Update,
        (
            update_player_count,
            handle_player_color_display,
            trigger_event_on_button_pressed::<StartGame>,
        )
            .run_if(in_state(AppState::Lobby)),
    );
    app.add_observer(start_game);
}

pub fn spawn_lobby(mut commands: Commands) {
    commands.spawn((Camera2d, PrimaryEguiContext, DespawnOnExit(AppState::Lobby)));
}

pub fn spawn_ui(mut commands: Commands) {
    commands.spawn((DespawnOnExit(AppState::Lobby), PlayerCount));

    commands.spawn((
        DespawnOnExit(AppState::Lobby),
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
        Name::new("PlayerColorDisplayContainer"),
        DespawnOnExit(AppState::Lobby),
        PlayerColorDisplayContainer,
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

pub fn start_game(_trigger: On<StartGame>, mut goto: MessageWriter<PleaseGoTo>) {
    info!("Sending GoTo(GameState::Playing) to server.");
    goto.write(PleaseGoTo(AppState::InGame));
}

#[derive(Component, Reflect)]
pub struct PlayerColorDisplayContainer;

#[derive(Component, Reflect)]
#[require(Text)]
pub struct PlayerCount;

pub fn update_player_count(
    mut text: Single<&mut Text, With<PlayerCount>>,
    n_clients: Query<&Player>,
) {
    text.0 = format!("{} players connected.", n_clients.count());
}

pub fn handle_player_color_display(
    mut commands: Commands,
    player_color_displays: Query<(Entity, &PlayerColor), (Without<Node>, With<ColorDisplay>)>,
    containers: Query<Entity, With<PlayerColorDisplayContainer>>,
) {
    if let Ok(container_entity) = containers.single() {
        for (entity, color) in player_color_displays {
            commands.entity(container_entity).add_child(entity);
            commands.entity(entity).insert((
                Name::new("PlayerColorDisplay"),
                Node {
                    width: percent(100),
                    height: percent(100),
                    ..default()
                },
                BackgroundColor(color.0),
            ));
        }
    }
}
