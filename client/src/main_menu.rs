use crate::{connect::ConnectClient, host::StartHostServer};
use bevy::prelude::*;
use shared::AppState;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), spawn_ui);
    app.add_systems(
        Update,
        (
            trigger_event_on_button_pressed::<StartHostServer>,
            trigger_event_on_button_pressed::<ConnectClient>,
        )
            .run_if(in_state(AppState::MainMenu)),
    );
}

pub fn spawn_ui(mut commands: Commands) {
    commands.spawn((DespawnOnExit(AppState::MainMenu), Camera2d::default()));

    commands.spawn((
        DespawnOnExit(AppState::MainMenu),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        children![
            text("Main Menu"),
            (text("Host"), StartHostServer, Button),
            (text("Connect"), ConnectClient, Button),
        ],
    ));
}

/// The type T has to be both an Event and a Component.
/// Queries for an entity with component T and triggers
/// the component if Interaction is pressed.
pub fn trigger_event_on_button_pressed<'a, E: Event<Trigger<'a>: Default> + Component + Clone>(
    mut commands: Commands,
    query: Query<(&Interaction, &E), Changed<Interaction>>,
) {
    for (interaction, event_component) in query {
        match *interaction {
            Interaction::Pressed => commands.trigger(event_component.clone()),
            _ => {}
        }
    }
}

pub fn text<S: Into<String>>(string: S) -> Text {
    Text::new(string)
}
