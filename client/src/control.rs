use bevy::prelude::*;
use shared::{GameState, control::GoTo};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, recieve_goto);
}

pub fn recieve_goto(mut goto: MessageReader<GoTo>, mut game_state: ResMut<NextState<GameState>>) {
    for state in goto.read() {
        game_state.set(state.0);
    }
}
